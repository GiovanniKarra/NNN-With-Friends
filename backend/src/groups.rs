use rocket::{http::CookieJar, serde::Serialize, State};
use sqlx::{Pool, Sqlite};

use crate::login::authenticate_session;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Group {
	pub groupid: i64,
	pub name: String,
	pub creator: String
}

#[post("/create/<group_name>", rank = 11)]
pub async fn create_group_route(db: &State<Pool<Sqlite>>, cookies: &CookieJar<'_>, group_name: &str)
	-> Result<String, String> {

	let user = authenticate_session(db, cookies).await.ok_or("User not found")?;
	
	create_group(db, &group_name.to_owned(), &user.username)
		.await
		.map(|group| group.groupid.to_string())
}

async fn create_group(pool: &Pool<Sqlite>, group_name: &String, founder: &String)
	-> Result<Group, String> {
	
	let id = rand::random::<i64>();

	sqlx::query!(
		"INSERT INTO groups (id, name, founder) VALUES (?, ?, ?);",
		id, group_name, founder
	)
	.execute(pool)
	.await
	.map(|_| Group {
		groupid: id, name: group_name.clone(), creator: founder.clone()
	})
	.map_err(|_| "Internal error. Couldn't create group.".to_owned())
}

#[post("/<groupid>/join")]
pub async fn group_join(db: &State<Pool<Sqlite>>, cookies: &CookieJar<'_>, groupid: i64)
	-> Result<String, String> {
	
	let user = authenticate_session(&db, cookies)
		.await
		.ok_or("Couldn't authenticate user".to_owned())?;

	group_add_user(&db, groupid, &user.username)
		.await
		.map(|_| "Joined group successfully!".to_owned())
}

#[post("/<groupid>/leave")]
pub async fn group_leave(db: &State<Pool<Sqlite>>, cookies: &CookieJar<'_>, groupid: i64)
	-> Result<String, String> {
	
	let user = authenticate_session(&db, cookies)
		.await
		.ok_or("Couldn't authenticate user".to_owned())?;

	group_remove_user(&db, groupid, &user.username)
		.await
		.map(|_| "Left group successfully!".to_owned())
}

async fn group_add_user(pool: &Pool<Sqlite>, groupid: i64, username: &String)
	-> Result<(), String> {
	
	sqlx::query!(
		"INSERT INTO group_membership (user, groupid) VALUES (?, ?);",
		username, groupid
	)
	.execute(pool)
	.await
	.map(|_| ())
	.map_err(|_| format!("Internal error. Couldn't add {} to group {}", username, groupid))
}

async fn group_remove_user(pool: &Pool<Sqlite>, groupid: i64, username: &String)
	-> Result<(), String> {
	
	sqlx::query!(
		"DELETE FROM group_membership WHERE user = ? AND groupid = ?;",
		username, groupid
	)
	.execute(pool)
	.await
	.map(|_| ())
	.map_err(|_| format!("Internal error. Couldn't remove {} from group {}", username, groupid))
}
