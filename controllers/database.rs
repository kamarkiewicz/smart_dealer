
use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use router::Router;
use rustc_serialize::json::{self, ToJson};
use ipm::PostgresReqExt;
use std::fs::File;
use std::io::Read;

fn init_db(req: &PostgresReqExt) -> Result<(), String> {
    // init database using db.sql
    let mut f = File::open("db.sql").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let conn = req.db_conn();
    match conn.batch_execute(&s[..]) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn init(req: &mut Request) -> IronResult<Response> {
    let resp = init_db(req);
    let data = btreemap! {
        "success".to_string() => resp.is_ok().to_json(),
        "error_msg".to_string() => resp.err().to_json()
    };
    let payload = json::encode(&data).unwrap();
    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::json());
    Ok(resp)
}

fn populate_db(req: &PostgresReqExt) -> Result<(), String> {
    // populate database using db_sample.sql
    let mut f = File::open("db_sample.sql").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let conn = req.db_conn();
    match conn.batch_execute(&s[..]) {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string())
    }
}

fn populate(req: &mut Request) -> IronResult<Response> {
    let resp = populate_db(req);
    let data = btreemap! {
        "success".to_string() => resp.is_ok().to_json(),
        "error_msg".to_string() => resp.err().to_json()
    };
    let payload = json::encode(&data).unwrap();
    let mut resp = Response::with((status::Ok, payload));
    resp.headers.set(ContentType::json());
    Ok(resp)
}

pub fn routes() -> Router {
    let mut router = Router::new();
    router
        .get("/init", init)
        .get("/populate", populate);
        // .post("/details/:client_id", details_post);
    router
}
