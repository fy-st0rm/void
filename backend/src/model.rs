use crate::schema::{
	users,
	files
};
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

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile <'a> {
	pub id: &'a str,
	pub owner: &'a str,
	pub name: &'a str,
	pub url: &'a str
}

#[derive(Queryable, Debug, AsChangeset)]
pub struct File {
	pub id: String,
	pub owner: String,
	pub name: String,
	pub url: String
}
