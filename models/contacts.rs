
#[derive(ToJson)]
pub struct Contact {
	pub contact_id: i32,
    pub forename: String,
    pub surname: String,
    pub details: Vec<ContactDetail>,
    pub created: String
}

#[derive(ToJson)]
pub struct ContactDetail {
    pub contact_detail_id: i32,
    pub contact_id: i32,
    pub detail_type: String,
    pub detail_value: String
}

impl Contact {
    pub fn get_all(conn: int) -> Vec<Contact> {
        let res = Vec<Contact>::new();
        
        let stmt = conn.execute("", &[]).unwrap();


        res
    }
}
