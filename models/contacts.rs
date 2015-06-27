
use ipm::PostgresReqExt;
use rustc_serialize::json::{self, ToJson};
use time::Timespec;

#[derive(ToJson)]
pub struct Contact {
	pub contact_id: i32,
    pub forename: String,
    pub surname: String,
    pub created: Timespec
}

#[derive(ToJson)]
pub struct ContactDetail {
    pub contact_detail_id: i32,
    pub contact_id: i32,
    pub detail_type: String,
    pub detail_value: String
}

impl Contact {
    pub fn get_all(req: &PostgresReqExt) -> Vec<Contact> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
                "SELECT contact_id,\
                        forename,\
                        surname FROM contacts"
            ).unwrap();
        let rows = stmt.query(&[]).unwrap();
        for row in rows {
            vec.push(Contact {
                contact_id: row.get(0),
                forename: row.get(1),
                surname: row.get(2),
                created: row.get(3)
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
                "SELECT contact_detail_id,\
                        contact_id,\
                        type,\
                        value\
                    FROM contact_details\
                    WHERE contact_id = $1"
            ).unwrap();
        let rows = stmt.query(&[&contact_id]).unwrap();
        for row in rows {
            vec.push(ContactDetail {
                contact_detail_id: row.get(0),
                contact_id: row.get(1),
                detail_type: row.get(2),
                detail_value: row.get(3)
            }); 
        }
        vec
    }

    pub fn save(&self, req: &PostgresReqExt) {
        let conn = req.db_conn();
        let _ = conn.execute(
                "SELECT \"contact_detail_save\"($1, $2, $3, $4)",
            &[&self.contact_detail_id,
              &self.contact_id,
              &self.detail_type,
              &self.detail_value]
            ).unwrap();
    }
}
