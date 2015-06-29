
use postgres;
use ipm::PostgresReqExt;
use rustc_serialize::json::{self, ToJson};
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

#[derive(ToJson)]
pub struct ContactDetail {
    pub contact_detail_id: i32,
    pub contact_id: i32,
    pub dtype: String,
    pub dvalue: String
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
}

impl ContactDetail {
    pub fn get_by_client_id(req: &PostgresReqExt, contact_id: i32) -> Vec<ContactDetail> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
            "SELECT contact_detail_id,contact_id,type,dvalue \
             FROM contact_details WHERE contact_id=$1"
            ).unwrap();
        let rows = stmt.query(&[&contact_id]).unwrap();
        for row in rows {
            vec.push(ContactDetail {
                contact_detail_id: row.get(0),
                contact_id: row.get(1),
                dtype: row.get(2),
                dvalue: row.get(3)
            }); 
        }
        vec
    }

    pub fn commit(&self, req: &PostgresReqExt) -> postgres::Result<u64> {
        let conn = req.db_conn();
        //TODO: trigger for INSERT or UPDATE to remove duplicates.
        //      if contact_detail_id is 0, then INSERT else UPDATE.
        conn.execute(
            "INSERT INTO contact_details  \
             VALUES($1, $2, $3, $4)       ",
            &[&self.contact_detail_id,
              &self.contact_id,
              &self.dtype,
              &self.dvalue]
            )
    }
}
