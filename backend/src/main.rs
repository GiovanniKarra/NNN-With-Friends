use std::{env, io, path::PathBuf};

use rocket::fs::{FileServer, NamedFile};

#[macro_use]
extern crate rocket;

mod login;
use login::{login as login_route, signup, logout};
use sqlx::SqlitePool;

#[get("/<_path..>", rank = 11)]
async fn index(_path: PathBuf) -> io::Result<NamedFile> {
	let dir = env::var("FRONTEND_DIR").unwrap_or("./".to_owned());
	NamedFile::open(dir + "index.html").await
}

#[launch]
async fn rocket() -> _ {
	let _ = dotenvy::dotenv();

	let dir = env::var("FRONTEND_DIR").unwrap_or("./".to_owned());

	let db_url = env::var("DATABASE_URL").unwrap();
	let pool = SqlitePool::connect(db_url.as_str()).await.unwrap();

	rocket::build()
		.mount("/", FileServer::from(dir))
		.mount("/", routes![index])
		.mount("/api", routes![login_route, signup, logout])
		.manage(pool)
}
