use crate::schema::users;
use diesel::{
	Insertable,
	Queryable,
	AsChangeset
};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
	pub id: &'a str,
	pub name: &'a str,
	pub hash: &'a str
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct User {
	pub id: String,
	pub name: String,
	pub hash: String
}

