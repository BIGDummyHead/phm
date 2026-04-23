mod worker;

use std::marker::PhantomData;

use smol::{
    LocalExecutor,
    channel::{self, SendError, Sender},
};

pub struct Manager<'f, Fut, FutResult>
where
    FutResult: Send,
    Fut: Future<Output = FutResult> + Send + 'f,
{
    sender: Sender<Fut>,
    phant: PhantomData<&'f ()>,
}

impl<'f, Fut, FutResult> Manager<'f, Fut, FutResult>
where
    FutResult: Send,
    Fut: Future<Output = FutResult> + Send + 'f,
{
    /// Creates a new manager from a thread scope.
    ///
    /// Spawning thread pools based on the thread count.
    pub fn new<'scope, 'env>(
        scope: &'scope std::thread::Scope<'scope, 'env>,
        thread_cnt: usize,
    ) -> Self
    where
        'env: 'scope,
        Fut: 'env,
    {
        let (sx, rx) = channel::bounded::<Fut>(1000);

        // for the thread count, spawn a scoped thread
        for _ in 0..thread_cnt {
            let rx = rx.clone();
            scope.spawn(move || {
                // create a new local executor for each thread
                let lex = LocalExecutor::new();

                //make this thread async by using a block on
                smol::block_on(lex.run(async {
                    while let Ok(data) = rx.recv().await {
                        lex.spawn(async move {
                            data.await;
                        })
                        .detach();
                    }
                }));
            });
        }

        Self {
            sender: sx,
            phant: PhantomData,
        }
    }

    /// Send the future
    pub async fn send_work(&self, data: Fut) -> Result<(), SendError<Fut>> {
        self.sender.send(data).await
    }
}
