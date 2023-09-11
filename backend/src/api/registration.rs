use crate::model::{
	NewUser,
	User
};
use crate::db::DbPool;
use crate::schema::users::dsl::*;
use crate::types::*;
use crate::api::auth::generate_jwt;
use crate::VResponse;

use uuid::Uuid;
use pwhash::bcrypt;
use diesel::prelude::*;
use actix_web::{
	web,
	HttpResponse
};

use std::collections::HashMap;

pub async fn signup(
	payload: web::Json<VSignupPayload>,
	pool: web::Data<DbPool>
) -> HttpResponse {
	let username: &String = &payload.name;
	let password: &String = &payload.password;

	let mut conn = pool.get().expect("Cannot get db connection from pool");

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
				("msg", format!("Account with name {} already exists.", username))
			]
		},

		Err(_) => {
			diesel::insert_into(users)
				.values(&new_user)
				.execute(&mut conn)
				.expect(&format!("Error saving new user: {}", username));

			VResponse![
				("msg", format!("Sucessfully created account: {}", username))
			]
		}
	};

	return res;
}

pub async fn login(
	payload: web::Json<VLoginPayload>,
	pool: web::Data<DbPool>
) -> HttpResponse {
	let username: &String = &payload.name;
	let password: &String = &payload.password;

	let mut conn = pool.get().expect("Cannot get db connection from pool");
	let result = users
		.filter(name.eq(username))
		.first::<User>(&mut conn);

	let res: HttpResponse = match result {
		Ok(user) => {
			if !bcrypt::verify(password, &user.hash) {
				return VResponse![
					("msg", "Invalid password.")
				];
			}

			let token: String = generate_jwt(user);
			VResponse![
				("token", &token)
			]
		},

		Err(_) => {
			VResponse![
				("msg", format!("Account with name {} doesnt exists.", username))
			]
		}
	};

	return res;
}
