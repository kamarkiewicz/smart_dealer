
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{Json, ToJson};
use ipm::PostgresReqExt;

use ::models;
use hbs::Template;
use std::collections::BTreeMap;

pub fn contacts(req: &mut Request) -> IronResult<Response> {
    
}
