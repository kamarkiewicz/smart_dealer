#![feature(custom_derive, plugin)]
#![plugin(tojson_macros)]

extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate iron_postgres_middleware as ipm;
extern crate handlebars_iron as hbs;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use middlewares::*;

mod models;
mod controllers;
mod middlewares;

const  DB_ADDRESS : &'static str = "postgres://dev:dev@localhost"; 
const APP_ADDRESS : &'static str = "localhost:3000";

fn main() {
    let mut chain = Chain::new(controllers::routes());
    chain.link_before(database::register());
    chain.link_after(handlebars::register());

    println!("Server is running at http://{}/", APP_ADDRESS);
    Iron::new(chain).http(APP_ADDRESS).unwrap();
}
