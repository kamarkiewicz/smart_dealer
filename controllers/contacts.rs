
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::ToJson;

use ::models;
use hbs::Template;
// use std::collections::BTreeMap;

fn contacts(req: &mut Request) -> IronResult<Response> {
    let v = models::Contact::get_all(req);
    let data = btreemap! {
        "title".to_string() => "Kontakty".to_json(),
        "contacts".to_string() => v.to_json()
    };
    Ok(Response::with((status::Ok, Template::new("contacts", data))))
}

pub fn routes() -> Router {
    let mut router = Router::new();
    router.get("/", contacts);
    // router.get("/details/:id", details_by_id)
    router
}
