use std::{env, io, path::PathBuf};

use chrono::NaiveDate;
use groups::{create_group_route, get_user_groups, group_get_status, group_join, group_leave};
use rocket::{fs::{FileServer, NamedFile}, serde::json::Json, State};
use sqlx::SqlitePool;

#[macro_use]
extern crate rocket;

mod login;
mod users;
mod groups;

use login::{login as login_route, signup, logout};
use users::{fail, user_status};


#[get("/<_path..>", rank = 11)]
async fn index(_path: PathBuf) -> io::Result<NamedFile> {
	let dir = env::var("FRONTEND_DIR").unwrap_or("./".to_owned());
	NamedFile::open(dir + "index.html").await
}

#[get("/timeWindow")]
async fn get_time_window(time_window: &State<(i64, i64)>) -> Json<(i64, i64)> {
	Json(*time_window.inner())
}

#[launch]
async fn rocket() -> _ {
	let _ = dotenvy::dotenv();

	let dir = env::var("FRONTEND_DIR").unwrap_or("./".to_owned());

	let db_url = env::var("DATABASE_URL").unwrap();
	let pool = SqlitePool::connect(db_url.as_str()).await.unwrap();

	let start_time_str = env::var("START_TIME").unwrap_or("011124".to_owned());
	let start_time = 
		NaiveDate::parse_from_str(&start_time_str, "%d%m%y")
		.map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
		.expect("parsing error, make sure your date is encoded as %d%m%y \
				(eg 010924 for the 1st of Septembre)");

	let end_time_str = env::var("END_TIME").unwrap_or("011224".to_owned());
	let end_time = 
		NaiveDate::parse_from_str(&end_time_str, "%d%m%y")
		.map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp())
		.expect("parsing error, make sure your date is encoded as %d%m%y \
				(eg 010924 for the 1st of Septembre)");

	rocket::build()
		.mount("/", FileServer::from(dir))
		.mount("/", routes![index])
		.mount("/api", routes![login_route, signup, logout, get_time_window])
		.mount("/api/users", routes![fail, user_status])
		.mount("/api/groups", routes![create_group_route, group_join, group_leave, group_get_status, get_user_groups])
		.manage(pool)
		.manage((start_time, end_time))
}
