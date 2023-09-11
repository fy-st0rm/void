use diesel::prelude::*;
use diesel::r2d2;
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> r2d2::ConnectionManager<SqliteConnection> {
	dotenv().ok();

	let database_url = std::env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	r2d2::ConnectionManager::<SqliteConnection>::new(&database_url)
}
