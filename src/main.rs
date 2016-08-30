
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();         // Alternative syntax:
    router.get("/", handler);               // let router = router!(get "/" => handler,
    router.get("/:query", handler);         // get "/:query" => handler);
    router.get("/hello", hello);            // get "/:query" => handler);

    Iron::new(router).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }

    fn hello(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Whoot!" )))
    }

}
 

