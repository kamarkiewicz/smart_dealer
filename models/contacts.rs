
use postgres;
use ipm::PostgresReqExt;
use rustc_serialize::json::ToJson;
use time::Timespec;

#[derive(ToJson)]
pub struct Contact {
	pub contact_id: i32,
    pub forename: String,
    pub surname: String,
    pub email: String,
    pub cell: String,
    pub description: String,
    pub modified: Timespec
}

impl Contact {
    pub fn get_all(req: &PostgresReqExt) -> Vec<Contact> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
                "SELECT * FROM contacts"
            ).unwrap();
        let rows = stmt.query(&[]).unwrap();
        for row in rows {
            vec.push(Contact {
                contact_id: row.get(0),
                forename: row.get(1),
                surname: row.get(2),
                email: row.get(3),
                cell: row.get(4),
                description: row.get(5),
                modified: row.get(6)
            }); 
        }
        vec
    }

    pub fn commit(&self, req: &PostgresReqExt) -> postgres::Result<u64> {
        let conn = req.db_conn();
        //TODO: trigger for INSERT or UPDATE to remove duplicates.
        //      if contact_id is 0, then INSERT else UPDATE.
        conn.execute(
            "INSERT INTO contacts               \
             VALUES($1, $2, $3, $4, $5, $6, $7) ",
            &[&self.contact_id,
              &self.forename,
              &self.surname,
              &self.email,
              &self.cell,
              &self.description,
              &self.modified]
            )
    }
}
