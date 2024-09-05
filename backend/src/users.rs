use std::time::UNIX_EPOCH;

use rocket::{http::CookieJar, serde::{json::Json, Serialize}, State};
use sqlx::{Pool, Sqlite};

use crate::login::authenticate_session;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
	pub username: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserStatus {
	pub username: String,
	pub failed_time: Option<i64>,
	pub failed_msg: Option<String>
}

#[post("/fail/<message>")]
pub async fn fail(cookies: &CookieJar<'_>, db: &State<Pool<Sqlite>>,
	time_window: &State<(i64, i64)>,
	message: &str) -> Option<Json<UserStatus>> {

	let username = authenticate_session(db, &cookies).await?.username;
	fail_user(db, username, message.to_owned(), time_window.0).await.map(|status| Json(status))
}

#[get("/<username>/status")]
pub async fn user_status(username: &str, db: &State<Pool<Sqlite>>) -> Result<Json<UserStatus>, String> {
	get_user(db, &username.to_owned())
	.await
	.map(|status| Json(status))
	.ok_or(format!("User '{}' not found", username))
}

pub async fn get_user(pool: &Pool<Sqlite>, username: &String) -> Option<UserStatus> {
	sqlx::query_as!(
		UserStatus,
		"SELECT username AS 'username!', failed_time, failed_msg FROM users WHERE username = ?;",
		username
	)
	.fetch_optional(pool)
	.await
	.unwrap_or(None)
}

async fn fail_user(pool: &Pool<Sqlite>, username: String, message: String, start_time: i64)
	-> Option<UserStatus> {

	let time = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs() as i64 - start_time;

	sqlx::query!(
		"UPDATE users SET failed_time = ?, failed_msg = ? WHERE username = ?;",
		time, message, username
	)
	.execute(pool)
	.await
	.ok()
	.map(|_| UserStatus {
		username, failed_time: Some(time), failed_msg: Some(message)
	})
}