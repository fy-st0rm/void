use crate::db::DbPool;
use crate::schema::users::dsl::*;
use crate::types::*;
use crate::VResponse;

use std::collections::HashMap;
use actix_web::{
	web,
	HttpResponse
};

pub async fn dashboard(
	info: web::Path<(String,)>,
	pool: web::Data<DbPool>
) -> HttpResponse {
	let user_id: &String = &info.into_inner().0;
	VResponse![("msg", format!("test {}", user_id))]
}

