
use ipm::PostgresReqExt;
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
                        surname,
                        created FROM contacts"
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

    pub fn get_details(req: &PostgresReqExt, contact_id: i32) -> Vec<ContactDetail> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
                "SELECT contact_detail_id,\
                        contact_id,\
                        detail_type,\
                        detail_value FROM contact_details"
            ).unwrap();
        let rows = stmt.query(&[]).unwrap();
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
}
