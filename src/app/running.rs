use std::{pin::Pin, sync::Arc};

use tokio::{
    net::TcpListener,
    sync::{RwLock, watch},
    task::JoinHandle,
};

use crate::{App, HttpRequest, app::Closed, router::Router, web::http_request::Parsers};

pub struct Running {
    background_task: JoinHandle<()>,
    poison: Pin<Box<dyn Future<Output = ()>>>,
    http_parser: Parsers,
}

impl<'app> App<'app, Running> {
    /// # Running
    ///
    /// Creates a new running app that will handle all incoming connections until its timely death.
    pub fn running(closed: App<'app, Closed>) -> App<'app, Running> {

        let http_parser = closed.state.http_parser.unwrap_or(Parsers::default());
        let router = Arc::new(closed.router);

        //create a sender and receiver for our poison and interception.
        let (sender, mut receiver) = watch::channel(false);

        // the poison future, simply signals to the background task to close.
        let poison = Box::pin(async move {
            let send_res = sender.send(true);

            if let Err(e) = send_res {
                dbg!(e);
            }
        });

        // a background task that will sele
        let client_ref = closed.client.clone();
        let background_task = tokio::task::spawn(async move {
            let listener = &*client_ref;

            loop {
                // either select to handle the connection
                // or a poison which will end this loop.
                tokio::select! {
                    _ = handle_connection(listener, &closed.router, &http_parser) => {

                    }
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
            router,
            state: Running {
                background_task,
                poison,
                http_parser: Parsers::HttpV1,
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
            state: Closed,
        }
    }
}


/// Resonsible for accepting a Tcp Stream from an incoming request.
/// 
/// Then allows the workers to parse into an HttpRequest
async fn handle_connection<'app>(listener: &TcpListener, router: &'app Router<'app>, parser: &'app Parsers) -> std::io::Result<()> {
    let (stream, socket) = listener.accept().await?;

    let stream = Arc::new(RwLock::new(stream));

    let req_future = HttpRequest::parse(parser, router, stream, socket).await;

    Ok(())
}
