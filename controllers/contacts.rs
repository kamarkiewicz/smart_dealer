
use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json::{self, ToJson};
use iron::headers::ContentType;

use ::models;
use hbs::Template;
// use std::collections::BTreeMap;

fn contacts(req: &mut Request) -> IronResult<Response> {
    let v = models::Contact::get_all(req);
    let data = btreemap! {
        "title".to_string() => "Kontakty".to_json(),
        "contacts".to_string() => v.to_json()
    };
    let payload = Template::new("contacts", data);
    Ok(Response::with((status::Ok, payload)))
}

fn extended_contact_get(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.get::<Router>().unwrap();
    let contact_id_opt = params.find("contact_id").unwrap();
    let contact_id = contact_id_opt.parse::<i32>().unwrap();
    let addresses_vec = models::Address::get_by_contact_id(req, contact_id);
    let details_vec = models::ContactDetail::get_by_contact_id(req, contact_id);
    let data = btreemap! {
        "contact_id".to_string() => contact_id.to_json(),
        "addresses".to_string() => addresses_vec.to_json(),
        "details".to_string() => details_vec.to_json()
    };
    let payload = json::encode(&data).unwrap();
    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::json());
    Ok(resp)
}

// fn details_post(req: &mut Request) -> IronResult<Response> {
//     let params = req.extensions.get::<Router>().unwrap();
//     let id = params.find("id").unwrap();
//     let v = models::ContactDetail::contact_client_id(req, id);
//     let data = btreemap! {
//         "contact_id".to_string() => id.to_json(),
//         "details".to_string() => v.to_json()
//     }.to_json();
//     Ok(Response::with((status::Ok, data)))
// }

pub fn routes() -> Router {
    let mut router = Router::new();
    router
        .get("/", contacts)
        .get("/:contact_id", extended_contact_get)
        // .post("/new", account_new)
        ;
    router
}
