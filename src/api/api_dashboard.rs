use crate::db::establish_connection;
use crate::schema::files::dsl::*;
use crate::model::File;
use crate::VResponse;
use diesel::prelude::*;

use std::collections::HashMap;
use actix_web::{
	web,
	HttpResponse,
	http::StatusCode
};

pub async fn api_dashboard(info: web::Path<(String,)>) -> HttpResponse {
	let user_id: String = info.into_inner().0;
	let mut conn = establish_connection();

	let result = files
		.filter(owner.eq(&user_id))
		.load::<File>(&mut conn);

	let res = match result {
		Ok(stored_files) => {
			let mut response = HashMap::new();
			for file in stored_files {
				response.insert(file.id, file.name);
			}

			HttpResponse::build(StatusCode::OK).json(response)
		},
		Err(err) => {
			VResponse![StatusCode::BAD_REQUEST, ("msg", format!("{}", err))]
		}
	};

	return res;
}

