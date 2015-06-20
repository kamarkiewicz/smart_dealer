
pub use ipm::{PostgresMiddleware, PostgresReqExt};

use std::fs::File;
use std::io::Read;
use ::DB_ADDRESS;

pub fn register() -> PostgresMiddleware {
    // postgres
    let pg = PostgresMiddleware::new(DB_ADDRESS);
    
    // init database using db.sql
    {
        let mut f = File::open("db.sql").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        let conn = pg.pool.get().unwrap();
        conn.batch_execute(&s[..]).unwrap_or_else(|e| {
            println!("Unable to import db.sql: {}", e);
        });
    }

    pg
}
