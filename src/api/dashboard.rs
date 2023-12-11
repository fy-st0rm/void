use crate::db::establish_connection;
use crate::schema::users::dsl::*;
use crate::schema::files::dsl::*;
use crate::model::{ NewFile, File };
use crate::types::*;
use crate::VResponse;
use diesel::prelude::*;

use std::collections::HashMap;
use actix_web::{
	web,
	HttpResponse
};

pub async fn fetch_user_files(mut db: PgConnection, user_id: &str) {
	let result = files
		.filter(owner.eq(user_id))
		.load::<File>(&mut db);
	println!("{:?}", result);
}

pub async fn dashboard(info: web::Path<(String,)>) -> HttpResponse {
	let user_id: String = info.into_inner().0;
	let conn = establish_connection();

	fetch_user_files(conn, &user_id).await;
	VResponse![("msg", format!("test {}", user_id))]
}

