
pub use hbs::{Template, HandlebarsEngine, Watchable};
use std::sync::Arc;
use time::Timespec;
use rustc_serialize::json;

pub fn register() -> Arc<HandlebarsEngine> {
    let hbs_ref = Arc::new(HandlebarsEngine::new("./views/", ".hbs.htm"));
    hbs_ref.watch();
    hbs_ref
}
