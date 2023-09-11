mod api;
mod db;
mod schema;
mod model;
mod types;

use db::{
	establish_connection,
	DbPool
};

use api::registration::{
	signup,
	login
};

use api::dashboard::dashboard;

use actix_web::{
	web,
	App,
	HttpServer,
	middleware::Logger
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	// Setting up env logger
	if std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", "actix_web=info");
	}
	env_logger::init();

	// Setting up connection
	let db_conn = establish_connection();
	let db_pool = DbPool::new(db_conn).expect("Failed to create Database pool");

	HttpServer::new( move || {
		App::new()
			.app_data(web::Data::new(db_pool.clone()))
			.wrap(Logger::default())
			.route("/signup", web::post().to(signup))
			.route("/login", web::post().to(login))
			.route("/dashboard/{user_id}", web::get().to(dashboard))
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}
