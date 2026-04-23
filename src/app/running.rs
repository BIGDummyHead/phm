use std::{pin::Pin, sync::Arc};

use tokio::{
    net::TcpListener,
    sync::{
        RwLock,
        watch::{self},
    },
    task::JoinHandle,
};

use crate::{
    App, HttpRequest, Resolution, app::Closed, manager::Manager, router::Router,
    web::http_request::Parsers,
};

pub struct Running {
    background_task: JoinHandle<()>,
    poison: Pin<Box<dyn Future<Output = ()>>>,
    http_parser: Parsers,
    manager: Arc<Manager>,
}

impl<'app> App<'app, Running>
where
    'app: 'static,
{
    /// # Running
    ///
    /// Creates a new running app that will handle all incoming connections until its timely death.
    pub async fn running(closed: App<'app, Closed>) -> App<'app, Running> {
        let http_parser = closed.state.http_parser.unwrap_or(Parsers::default());
        let manager = Arc::new(Manager::new());
        //create a sender and receiver for our poison and interception.
        let (sender, mut receiver) = watch::channel(false);

        // the poison future, simply signals to the background task to close.
        let poison = Box::pin(async move {
            let send_res = sender.send(true);

            if let Err(e) = send_res {
                dbg!(e);
            }
        });

        let client_ref = closed.client.clone();
        let listener_router = closed.router.clone();
        let task_http_parser = http_parser.clone();
        let task_manager = manager.clone();

        let background_task = tokio::task::spawn(async move {
            loop {
                tokio::select! {
                    _ = handle_connection(client_ref.clone(), listener_router.clone(), task_http_parser.clone(), task_manager.clone()) => {}

                    _ = receiver.changed() => {
                        if *receiver.borrow() {
                            dbg!("receiver poisoned!");
                            break;
                        }
                    }
                }
            }
        });

        Self {
            client: closed.client,
            router: closed.router,
            state: Running {
                background_task,
                poison,
                http_parser,
                manager,
            },
        }
    }

    /// # Close
    ///
    /// Closes the current running app and replaces it with a closed app.
    pub async fn close(self) -> App<'app, Closed> {
        //poison, signals to shutdown the background task
        self.state.poison.await;
        //await till death :0

        let _ = self.state.background_task.await;

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
    listener: Arc<TcpListener>,
    router: Arc<RwLock<Router<'app>>>,
    parser: Parsers,
    manager: Arc<Manager>,
) -> std::io::Result<()> {
    let (stream, socket) = listener.accept().await?;

    let stream = Arc::new(RwLock::new(stream));

    let work = Box::pin(async move {
        let router_ref = router.read().await;
        let parse = HttpRequest::parse(&parser, &router_ref, stream, socket).await;

        match parse {
            Err(e) => {
                dbg!(e);
            }
            Ok(mut req) => {
                let node = req.node().clone();
                let node_guard = node.read().await;

                if let Some(func) = node_guard.request_fn() {
                    let mut res = Resolution::new();

                    // loop over each middleware item, if middleware indicates stop. Return
                    for cmw in node_guard.middleware() {
                        match cmw(&mut req, &mut res).await {
                            crate::web::Middleware::Stop => return,
                            crate::web::Middleware::Next => continue,
                        }
                    }

                    //call the resolution function
                    if let Err(e) = func(&mut req, &mut res).await {
                        dbg!(e);
                    }
                }
            }
        }
    });

    if let Err(e) = manager.send_work(work).await {
        dbg!(e);
    }

    Ok(())
}
