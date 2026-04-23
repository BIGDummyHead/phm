mod worker;

use std::{pin::Pin, sync::Arc};

use tokio::sync::{
    Mutex,
    mpsc::{self, Sender, error::SendError},
};

//pub use worker::Worker;

pub type SenderData = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
pub struct Manager {
    sender: Sender<SenderData>,
}

impl Manager
{
    /// # new
    ///
    /// Creates `n` workers to consume incoming work.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<SenderData>(1000);
        let receiver = Arc::new(Mutex::new(receiver));

        // create n workers to work
        for _ in 0..num_cpus::get() {
            //clone a receiver that can receive some type of work
            let rx = receiver.clone();
            tokio::spawn(async move {
                loop {
                    //instantly receive work
                    let work = rx.lock().await.recv().await;

                    // consume work
                    if let Some(work) = work {
                        work.await;
                    } else {
                        dbg!("Channel closed");
                        break;
                    };
                }
            });
        }

        Self { sender }
    }

    /// sends work to the workers. 
    /// 
    /// if sending fails, then a send error is returned with the value you tried to send
    pub async fn send_work(&self, work: SenderData) -> Result<(), SendError<SenderData>> {
        self.sender.send(work).await
    }
}
