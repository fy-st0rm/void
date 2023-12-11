use log::debug;
use actix_multipart::Multipart;
use actix_web::{ web, HttpRequest, HttpResponse, Error, http::StatusCode };
use futures_util::TryStreamExt as _;
use uuid::Uuid;
use std::io::Write;
use diesel::prelude::*;
use std::collections::HashMap;

use crate::VResponse;
use crate::types::VUploadPayload;
use crate::db::establish_connection;
use crate::schema::files::dsl::*;
use crate::model::{ NewFile, File };

pub async fn api_upload_register(payload: web::Json<VUploadPayload>) -> HttpResponse {
	let owner_id: &String = &payload.owner_id;
	let filename: &String = &payload.filename;

	let mut conn = establish_connection();
	let file_id = Uuid::new_v4().to_string();

	let new_file: NewFile = NewFile {
		id: &file_id,
		owner: &owner_id,
		name: &filename
	};

	let res = diesel::insert_into(files)
		.values(&new_file)
		.execute(&mut conn);

	match res {
		Ok(_) => {
			VResponse![
				StatusCode::OK,
				("msg", "Sucessfully registered file."),
				("file_id", &file_id)
			]
		},
		Err(e) => {
			VResponse![
				StatusCode::BAD_REQUEST,
				("msg", format!("{}", e))
			]
		}
	}
}

pub async fn api_upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
	while let Some(mut field)= payload.try_next().await? {
		let cd = field.content_disposition();

		let filename = cd
			.get_filename()
			.map_or_else(|| Uuid::new_v4().to_string(), sanitize_filename::sanitize);

		let filepath = format!("./storage/{}", filename);

		debug!("Saving {:?}", filepath);
		let mut f = web::block(|| std::fs::File::create(filepath)).await??;

		// Iterate over bytes of field
		while let Some(chunk) = field.try_next().await? {
			f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
		}
	}

	Ok(HttpResponse::Ok().body("Sucessfully uploaded file."))
}

pub async fn api_download_file(req: HttpRequest, info: web::Path<(String,)>) -> HttpResponse {
	let file_id: String = info.into_inner().0;
	let mut conn = establish_connection();

	let result = files
		.filter(id.eq(&file_id))
		.first::<File>(&mut conn);

	match result {
		Ok(file) => {
			let filepath = format!("./storage/{}", file.name);
			let file = actix_files::NamedFile::open_async(filepath).await;

			match file {
				Ok(f) => {
					f.into_response(&req)
				},
				Err(err) => {
					VResponse![
						StatusCode::BAD_REQUEST,
						("msg", format!("{}", err))
					]
				}
			}
		},
		Err(err) => {
			VResponse![
				StatusCode::BAD_REQUEST,
				("msg", format!("{}", err))
			]
		}
	}
}
