# Project Hail Mary

Project Hail Mary (PHM) is my second iteration at an attempt to make an ExpressJS like Rust Web Server.

PHM uses a multi-threaded async environment with SMOL to create an extremely lightweight and affordable Web Server while giving you the syntax of something like ExpressJS.

Benefits of using PHM

* Lightweight
* Scoped async runtime and threading
* Uses thread pooling and parallelism
* Easy to read and understand API
* Very maintainable
* Proc Macro support for postman and routing

## Getting Started

This example uses the Async wrapper crate [smol](https://crates.io/crates/smol)

Note: The feature `json` should be enabled for resultful APIs

```rs

use phm::{App, app::ClosedAppExt, middleware, request};

//start async runtime
fn main() { smol::block_on(amain()); }

async fn amain() {
  let socket = "127.0.0.1:80";
  // the app is now bound to the socket, but the app is not started.
  let app = App::bind(socket).await.expect("failed to bind app");

  // note, this will panic if "/api/user" with the method GET is found elsewhere.
  app.get("/api/user", middleware!(),
    request!(|req, res| {
      res.status(200).text("User information");
      Ok(()) // The request body uses a Result<(), RequestError). You can implement From<E> for RequestError for early try ? returns.
    })
  ).await;

  //alternative adding
  app.add_def(post_user).await.expect("route already existed");

  //app state changes to running, not much can be done other than calling the close() function.
  let app = app.start();

  loop { }
}

//alternatively with the macro crate `phm_pm` we can make something like:
#[phm_pm::route(route = "/api/user", method="POST")]
async fn post_user(req: &mut HttpRequest<'_>, res: &mut Response) -> Result<(), RequestError> {
  res.status(201).text("you created a user");
  Ok(())
}

```

## PHM Proc Macros

With the proc macro crate `phm_pm` you may use the following macros:

* route(route, method, middleware, module)
* postman
* postman_module
* postman_info!(collection_name)

Using the postman and attribute macros will provide useful when creating an API that can be tested by Postman. Here is how to use it:

```rs

#[phm_pm::postman_module]
mod users {
    /// # post_user
    ///
    /// Allows you to create a user!
    #[phm_pm::postman] 
    #[phm_pm::route(route = "/api/user", method = "POST")] //required for postman
    #[allow(dead_code)]
    async fn post_user(_req: &mut HttpRequest<'_>, res: &mut Response) -> Result<(), RequestError> {
        res.status(200).text("You did it!");
        Ok(())
    }
}
```

A folder `postman` is generated along with a file `postman_api.json` which contains data about your API and can be imported to POSTMAN.

