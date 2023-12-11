/*
use crate::model::{
	NewUser,
	User
};
use crate::db::DbPool;
use crate::schema::users::dsl::*;

use diesel::prelude::*;
use actix_web::{
	web,
	HttpResponse
};

use serde::{
	Serialize,
	Deserialize
};

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CreateUserPost {
	pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct Response {
	pub msg: String
}

pub async fn create_user(
	payload: web::Json<CreateUserPost>,
	pool: web::Data<DbPool>
) -> HttpResponse {
	let usr_name: &String = &payload.name;
	let mut conn = pool.get().expect("Cannot get db connection from pool");

	let new_user: NewUser = NewUser {
		name: &usr_name
	};

	diesel::insert_into(users)
		.values(&new_user)
		.execute(&mut conn)
		.expect(&format!("Error saving new user: {}", usr_name));

	let resp: Response = Response {
		msg: format!("Sucessfully added user: {}", usr_name)
	};
	HttpResponse::Ok().json(resp)
}

pub async fn get_all_user(
	pool: web::Data<DbPool>
) -> HttpResponse {
	let mut conn = pool.get().expect("Cannot get db connection from pool");

	let results = users
		.load::<User>(&mut conn)
		.expect("Failed to query users.");

	let mut resp = HashMap::new();
	for user in results {
		resp.insert(user.id, String::from(user.name));
	}

	HttpResponse::Ok().json(resp)
}

*/
