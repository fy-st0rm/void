mod api;
mod db;
mod schema;
mod model;
mod types;
mod web_routes;

use log::{
	info,
	debug,
	error
};
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
use api::dashboard::dashboard;
use web_routes::{
	index,
	login,
	signup
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

	Cors::permissive();

	info!("Server binded on {}:{}", SERVER_IP, SERVER_PORT);
	HttpServer::new( move || {
		App::new()
			.wrap(Logger::default())

			// Web routes
			.route("/", web::get().to(index))
			.route("/login", web::get().to(login))
			.route("/signup", web::get().to(signup))

			// Api routes
			.route("/api_signup", web::post().to(api_signup))
			.route("/api_login", web::post().to(api_login))
			.route("/dashboard/{user_id}", web::get().to(dashboard))
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}
