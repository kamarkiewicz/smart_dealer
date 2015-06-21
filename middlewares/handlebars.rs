
pub use hbs::{Template, HandlebarsEngine, Watchable};
use std::sync::Arc;

pub fn register() -> Arc<HandlebarsEngine> {
    let hbs_ref = Arc::new(HandlebarsEngine::new("./views/", ".hbs.htm"));
    hbs_ref.watch();
    hbs_ref
}
