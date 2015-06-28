
use ipm::PostgresReqExt;
use rustc_serialize::json::{self, ToJson};

#[derive(ToJson)]
pub struct Address {
    address_id: i32,
    address: String,
    postal_code: String,
    city: String,
    state: String,
    country: String,
    geospot: json::Json
}

impl Address {
    pub fn get_by_contact_id(req: &PostgresReqExt, contact_id: i32) -> Vec<Address> {
        let mut vec = Vec::new(); 
        let conn = req.db_conn();
        let stmt = conn.prepare(
            "SELECT * FROM addresses a \
             WHERE EXISTS(SELECT * \
                          FROM many_contacts_has_many_addresses b \
                          WHERE b.address_id_addresses=a.address_id \
                            AND b.contact_id_contacts=$1)"
            ).unwrap();
        let rows = stmt.query(&[&contact_id]).unwrap();
        for row in rows {
            vec.push(Address {
                address_id: row.get(0),
                address: row.get(1),
                postal_code: row.get(2),
                city: row.get(3),
                state: row.get(4),
                country: row.get(5),
                geospot: row.get(6)
            }); 
        }
        vec
    }
}
