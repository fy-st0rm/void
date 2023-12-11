use crate::model::{
	NewUser,
	User
};
use crate::db::establish_connection;
use crate::schema::users::dsl::*;
use crate::types::*;
use crate::VResponse;

use log::debug;
use uuid::Uuid;
use pwhash::bcrypt;
use diesel::prelude::*;
use actix_web::{
	web,
	HttpResponse,
	http::StatusCode
};

use std::collections::HashMap;

pub async fn api_signup(payload: web::Json<VSignupPayload>) -> HttpResponse {
	let username: &String = &payload.name;
	let password: &String = &payload.password;

	let mut conn = establish_connection();

	let pw_hashed = bcrypt::hash(password)
		.expect("Failed to hash password");

	let new_user: NewUser = NewUser {
		id: &Uuid::new_v4().to_string(),
		name: &username,
		hash: &pw_hashed
	};

	// Quering in database and checking if the username already exists or not
	let result = users
		.filter(name.eq(username))
		.first::<User>(&mut conn);

	let res: HttpResponse = match result {
		Ok(_) => {
			VResponse![
				StatusCode::BAD_REQUEST,
				("msg", format!("Account with name {} already exists.", username))
			]
		},

		Err(_) => {
			diesel::insert_into(users)
				.values(&new_user)
				.execute(&mut conn)
				.expect(&format!("Error saving new user: {}", username));

			VResponse![
				StatusCode::OK,
				("msg", format!("Sucessfully created account: {}", username))
			]
		}
	};

	return res;
}

pub async fn api_login( payload: web::Json<VLoginPayload>) -> HttpResponse {
	let username: &String = &payload.name;
	let password: &String = &payload.password;

	let mut conn = establish_connection();
	let result = users
		.filter(name.eq(username))
		.first::<User>(&mut conn);

	let res: HttpResponse = match result {
		Ok(user) => {
			if !bcrypt::verify(password, &user.hash) {
				return VResponse![
					StatusCode::BAD_REQUEST,
					("msg", "Invalid password.")
				];
			}

			VResponse![
				StatusCode::OK,
				("id", &user.id),
				("name", &user.name)
			]
		},

		Err(_) => {
			VResponse![
				StatusCode::BAD_REQUEST,
				("msg", format!("Account with name {} doesnt exists.", username))
			]
		}
	};

	return res;
}
