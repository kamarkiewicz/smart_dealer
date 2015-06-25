
use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use hbs::Template;

mod contacts;
// mod offers;
// mod products;
// mod deals;
// mod bank_accounts;

fn index(_: &mut Request) -> IronResult<Response> {
    let data = btreemap! {
        "title".to_string() => "Strona główna".to_string()
    };
    Ok(Response::with((status::Ok, Template::new("index", data))))
}

pub fn routes() -> Mount {
    let mut mount = Mount::new();
    mount
        .mount("/contacts/", contacts::routes())
        // .mount("/offers/", offers::routes())
        // .mount("/products/", products::routes())
        // .mount("/deals/", deals::routes())
        // .mount("/bank_accounts/", bank_accounts::routes())
        .mount("/static/", Static::new(Path::new("static")))
        .mount("/", index);

    mount
}
