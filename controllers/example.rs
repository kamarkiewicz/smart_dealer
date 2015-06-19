
use iron::prelude::*;
use iron::status;
use rustc_serialize::json::{Json, ToJson};
use ipm::PostgresReqExt;

use ::models;
use hbs::Template;
use std::collections::BTreeMap;

pub fn name_list(req: &mut Request) -> IronResult<Response> {
    let conn = req.db_conn();
    let stmt = conn.prepare("SELECT id, name FROM names").unwrap();
    let rows = stmt.query(&[]).unwrap();

    let mut resp_str = "Names:\n".to_string();

    for row in rows {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let name_format = format!("{}: {}\n", id, name);
        resp_str.push_str(&name_format);
    }

    Ok(Response::with((status::Ok, resp_str)))
}


fn make_data() -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();

    data.insert("year".to_string(), "2015".to_json());

    let teams = vec![ models::Team { name: "Jiangsu Sainty".to_string(),
                             pts: 43u16 },
                      models::Team { name: "Beijing Guoan".to_string(),
                             pts: 27u16 },
                      models::Team { name: "Guangzhou Evergrand".to_string(),
                             pts: 22u16 },
                      models::Team { name: "Shandong Luneng".to_string(),
                             pts: 12u16 } ];

    data.insert("teams".to_string(), teams.to_json());
    data
}


/// the handler
pub fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = make_data();
    resp.set_mut(Template::new("hello_world", data)).set_mut(status::Ok);
    Ok(resp)
}
