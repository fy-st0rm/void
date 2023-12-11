mod api;
mod db;
mod schema;
mod model;
mod types;
mod web_routes;

use log::info;
use std::fs;
use actix_files as afs;
use actix_web::{
	web,
	App,
	HttpServer,
	middleware::Logger,
};
use actix_cors::Cors;

use api::registration::{
	api_signup,
	api_login
};
use api::api_dashboard::api_dashboard;
use api::file_transfer::{
	api_upload_register,
	api_upload_file
};
use web_routes::{
	index,
	login,
	signup,
	dashboard
};


const SERVER_IP: &str  = "127.0.0.1";
const SERVER_PORT: u16 = 3000;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

	// Setting up env logger
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "info,void=error,void=debug");
	}
	env_logger::init();

	// Creating storage directory
	let _ = fs::create_dir("./storage");

	Cors::permissive();

	info!("Server binded on {}:{}", SERVER_IP, SERVER_PORT);
	HttpServer::new( move || {
		App::new()
			.wrap(Logger::default())

			// Serving static files
			.service(
				afs::Files::new("/templates", "./templates")
					.show_files_listing()
			)

			// Web routes
			.route("/", web::get().to(index))
			.route("/login", web::get().to(login))
			.route("/signup", web::get().to(signup))
			.route("/dashboard", web::get().to(dashboard))

			// Api routes
			.route("/api_signup", web::post().to(api_signup))
			.route("/api_login", web::post().to(api_login))
			.route("/api_dashboard/{user_id}", web::get().to(api_dashboard))
			.route("/api_upload_register", web::post().to(api_upload_register))
			.route("/api_upload_file", web::post().to(api_upload_file))
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}
