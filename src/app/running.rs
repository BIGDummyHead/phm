use std::pin::Pin;

use tokio::{net::TcpListener, sync::watch, task::JoinHandle};

use crate::{App, app::Closed};

pub struct Running {
    background_task: JoinHandle<()>,
    poison: Pin<Box<dyn Future<Output = ()>>>,
}

impl<'app> App<'app, Running> {
    /// # Running
    ///
    /// Creates a new running app that will handle all incoming connections until its timely death.
    pub fn running(closed: App<'app, Closed>) -> App<'app, Running> {
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
                    _ = handle_connection(listener) => {

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
            router: closed.router,
            state: Running {
                background_task,
                poison,
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

async fn handle_connection(listener: &TcpListener) -> std::io::Result<()>
{
    let (stream, socket) = listener.accept().await?;


    Ok(())
}
