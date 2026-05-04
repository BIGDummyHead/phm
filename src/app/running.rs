//! # Running
//!
//! Implements the [`Running`] lifecycle state of [`App`]. Spawns a dedicated
//! background thread that hosts a scoped thread pool (managed by
//! [`Manager`](crate::manager)) and dispatches incoming TCP connections to
//! worker threads, parses them using the configured HTTP parser, invokes any
//! registered middleware, and finally runs the request closure for the
//! matched route.

use std::{pin::Pin, sync::Arc};

use futures::{FutureExt, select};
use smol::{
    channel::{self, Receiver},
    io::AsyncWriteExt,
    lock::RwLock,
    net::{TcpListener, TcpStream},
};

use crate::{
    App, HttpRequest, Response, app::Closed, manager::Manager, router::Router,
    web::http_request::Parsers,
};

/// Represents a Running state of an application
pub struct Running {
    poison: Pin<Box<dyn Future<Output = ()>>>,
    http_parser: Parsers,
}

impl<'app> App<'app, Running> {
    /// # Running
    ///
    /// Creates a new running app that will handle all incoming connections until its timely death.
    pub fn run(closed: App<'app, Closed>) -> App<'app, Running> {
        let http_parser = closed.state.http_parser.unwrap_or(Parsers::default());
        //create a sender and receiver for our poison and interception.
        let (poison_sender, poison_receiver) = channel::bounded(1);

        // the poison future, simply signals to the background task to close.
        let poison = Box::pin(async move {
            if let Err(e) = poison_sender.send(true).await {
                dbg!(e);
            }
        });

        let listener_ref = closed.client.clone();
        let listener_router = closed.router.clone();
        let task_http_parser = http_parser.clone();

        Self::spawn_background_thread(
            poison_receiver,
            task_http_parser,
            listener_ref,
            listener_router,
        );

        Self {
            client: closed.client,
            router: closed.router,
            state: Running {
                poison,
                http_parser,
            },
        }
    }

    fn spawn_background_thread(
        poison_receiver: Receiver<bool>,
        task_http_parser: Parsers,
        listener_ref: Arc<TcpListener>,
        listener_router: Arc<RwLock<Router<'app>>>,
    ) where
        'app: 'static,
    {
        std::thread::spawn(|| {
            std::thread::scope(|scope| {
                let thread_count: usize = std::thread::available_parallelism().unwrap().into();

                let manager = Manager::new(scope, thread_count - 1);

                smol::block_on(async move {
                    loop {
                        let accepted = select! {
                            tcp_acception = listener_ref.accept().fuse() => {
                                tcp_acception
                            },
                            recv = poison_receiver.recv().fuse() => {
                                if let Ok(v) = recv && v {
                                    break;
                                }
                                else {
                                    continue;
                                }
                            }
                        };

                        let (stream, socket) = match accepted {
                            Err(e) => {
                                dbg!(e);
                                break;
                            }
                            Ok(v) => v,
                        };

                        let work = handle_connection(
                            stream,
                            socket,
                            listener_router.clone(),
                            task_http_parser.clone(),
                        );

                        if let Err(e) = manager.send_work(work).await {
                            dbg!("failure to send work", e);
                        }
                    }
                });
            });
        });
    }

    /// # Close
    ///
    /// Closes the current running app and replaces it with a closed app.
    pub async fn close(self) -> App<'app, Closed> {
        //poison, signals to shutdown the background task
        self.state.poison.await;

        App {
            client: self.client,
            router: self.router,
            state: Closed {
                http_parser: Some(self.state.http_parser),
            },
        }
    }
}

/// Resonsible for accepting a Tcp Stream from an incoming request.
///
/// Then allows the workers to parse into an HttpRequest
async fn handle_connection<'app>(
    stream: TcpStream,
    socket: std::net::SocketAddr,
    router: Arc<RwLock<Router<'app>>>,
    parser: Parsers,
) -> std::io::Result<()> {
    let stream = Arc::new(RwLock::new(stream));

    let parse = HttpRequest::parse(&parser, router.clone(), stream.clone(), socket).await;
    let response = match parse {
        Err(e) => {
            dbg!(e);

            let mut res = Response::new(&parser);
            res.status(404);
            res.set_header("Content-Length", "0");

            res
        }
        Ok(mut req) => {
            let mut res = Response::new(&parser);

            let node = req.node().clone();
            let node_guard = node.read().await;

            if let Some(func) = node_guard.request_fn() {
                let mut middleware_stops = false;
                // loop over each middleware item, if middleware indicates stop. Return
                for cmw in node_guard.middleware() {
                    match cmw(&mut req, &mut res).await {
                        crate::web::Middleware::Stop => {
                            middleware_stops = true;
                            break;
                        }
                        crate::web::Middleware::Next => continue,
                    }
                }

                // only call the request function if the middleware did NOT stop.
                if !middleware_stops {
                    //call the resolution function
                    let func = func.clone();
                    let result = (*func)(&mut req, &mut res).await;

                    if let Err(e) = result {
                        e.handle(&mut res);
                    }
                }
            }

            res
        }
    };

    let mut stream_write_guard = stream.write().await;

    let write: Arc<[u8]> = response.into();
    stream_write_guard.write_all(&write).await?;
    stream_write_guard.flush().await?;

    Ok(())
}
