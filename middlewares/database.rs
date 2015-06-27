
pub use ipm::{PostgresMiddleware, PostgresReqExt};

use ::DB_ADDRESS;

pub fn register() -> PostgresMiddleware {
    // postgres
    let pg = PostgresMiddleware::new(DB_ADDRESS);
    pg
}
