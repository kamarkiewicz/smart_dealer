
use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

use self::example::*;

pub mod example;

pub fn routes() -> Mount {
    let mut router = Router::new();
    router
        .get("/", hello_world)
        .get("/nl", name_list);

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/static/", Static::new(Path::new("static")));

    mount
}
