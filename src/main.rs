
extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use router::NoRoute;

fn main() {
    let mut router = Router::new();         // Alternative syntax:
//  router.get("/", handler);               // let router = router!(get "/" => handler,
//  router.get("/:query", handler);         // get "/:query" => handler);
    router.get("/hello", hello);            // get "/:query" => handler);
    router.get(r"/.*", notfound);            // get "/:query" => handler);

// raw strings, so they don't need escaping...
// router.get(r"/(\d+)", digit_handler);
// router.route("items", notfound);
// let _ = 3000;

    println!("starting server on 3000"); 

    Iron::new(router).http("localhost:3000").unwrap();

    //router.finalize().unwrap();

/*
    fn handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
*/
    fn hello(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Whoot!" )))
    }

    fn notfound(_: &mut Request) -> IronResult<Response> {
        println!("not found returning 404"); 
        Err(IronError::new(NoRoute, status::NotFound))
    }

}
 

