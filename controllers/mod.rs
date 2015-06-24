
use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

mod contacts;
// mod offers;
// mod products;
// mod deals;
// mod bank_accounts;

fn home_page(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
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
        .mount("/", home_page);

    mount
}
