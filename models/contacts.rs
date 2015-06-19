#[derive(ToJson)]
pub struct Contact {
	pub contact_id: i32,
    pub forename: String,
    pub surname: String,
    pub created: String
}

#[derive(ToJson)]
pub struct Team {
    pub name: String,
    pub pts: u16
}
