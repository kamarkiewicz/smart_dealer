
use postgres;
use ipm::PostgresReqExt;
use rustc_serialize::json::ToJson;
use time::Timespec;

#[derive(ToJson)]
pub struct ContactDetail {
    pub contact_detail_id: i32,
    pub contact_id: i32,
    pub dtype: String,
    pub dvalue: String
}

impl ContactDetail {
    pub fn get_by_contact_id(req: &PostgresReqExt, contact_id: i32) -> Vec<ContactDetail> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
            "SELECT * FROM contact_details  \
             WHERE contact_id=$1            "
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
