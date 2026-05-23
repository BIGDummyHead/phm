//! # Manager
//!
//! Scoped thread-pool manager used by the running [`App`](crate::App) to
//! dispatch incoming connections. Each worker thread owns a
//! [`LocalExecutor`](smol::LocalExecutor) and receives futures over a bounded
//! channel, so that the producer loop on the accept thread can hand off
//! work without blocking.

use std::{marker::PhantomData, sync::Arc};

use smol::channel::{self, SendError, Sender};

/// # Manager
///
/// Bounded-channel handle to a scoped thread pool. Each worker thread
/// `await`s on the channel for incoming futures and drives them to
/// completion on a thread-local [`smol::LocalExecutor`].
///
/// The `'f` lifetime is the minimum lifetime required by the futures that
/// will be sent through the channel; `FutResult` is the future's output
/// type (required to be `Send`).
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
        let ex = Arc::new(smol::Executor::new());
        for _ in 0..thread_cnt {
            let ex = ex.clone();
            let rx = rx.clone();
            scope.spawn(move || {
                // create a new local executor for each thread
                //make this thread async by using a block on
                smol::block_on(ex.run(async {
                    while let Ok(data) = rx.recv().await {
                        ex.spawn(async move {
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
