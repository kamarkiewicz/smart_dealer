
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::{self, ToJson};

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

fn details_get(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let id_opt = params.find("id").unwrap();
    let id = id_opt.parse::<i32>().unwrap();
    let v = models::ContactDetail::get_by_client_id(req, id);
    let data = btreemap! {
        "client_id".to_string() => id.to_json(),
        "details".to_string() => v.to_json()
    };
    let payload = json::encode(&data).unwrap();
    Ok(Response::with((status::Ok, payload)))
}

// fn details_post(req: &mut Request) -> IronResult<Response> {
//     let params = req.extensions.get::<Router>().unwrap();
//     let id = params.find("id").unwrap();
//     let v = models::ContactDetail::get_by_client_id(req, id);
//     let data = btreemap! {
//         "client_id".to_string() => id.to_json(),
//         "details".to_string() => v.to_json()
//     }.to_json();
//     Ok(Response::with((status::Ok, data)))
// }

pub fn routes() -> Router {
    let mut router = Router::new();
    router
        .get("/", contacts)
        .get("/details/:id", details_get);
        // .post("/details/:id", details_post);
        // .get("/details/:id", details_by_id)
    router
}
