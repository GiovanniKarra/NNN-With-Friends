use rocket::{http::CookieJar, serde::{json::Json, Serialize}, State};
use sqlx::{Pool, Sqlite};

use crate::{login::authenticate_session, users::UserStatus};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Group {
	pub id: i64,
	pub name: String,
	pub founder: String
}

#[post("/create/<group_name>", rank = 11)]
pub async fn create_group_route(db: &State<Pool<Sqlite>>, cookies: &CookieJar<'_>, group_name: &str)
	-> Result<String, String> {

	let user = authenticate_session(db, cookies).await.ok_or("User not found")?;
	
	create_group(db, &group_name.to_owned(), &user.username)
		.await
		.map(|group| group.id.to_string())
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
		id, name: group_name.clone(), founder: founder.clone()
	})
	.map_err(|err| format!("Internal error. Couldn't create group. {}", err.to_string()))
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

#[get("/<groupid>/status")]
pub async fn group_get_status(db: &State<Pool<Sqlite>>, groupid: i64) -> Result<Json<Vec<UserStatus>>, String> {
	group_get_all_users(db, groupid)
		.await
		.map(|u| Json(u))
}

#[get("/myGroups")]
pub async fn get_user_groups(db: &State<Pool<Sqlite>>, cookies: &CookieJar<'_>) -> Result<Json<Vec<Group>>, String> {
	let username = authenticate_session(db, cookies)
		.await
		.ok_or("Couldn't authenticate user")?
		.username;

	groups_from_user(db, &username)
		.await
		.map(|v| Json(v))
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
		"
		DELETE FROM group_membership
		WHERE user = ? AND groupid = ?;
		",
		username, groupid
	)
	.execute(pool)
	.await
	.map(|_| ())
	.map_err(|_| format!("Internal error. Couldn't remove {} from group {}", username, groupid))
}

async fn group_get_all_users(pool: &Pool<Sqlite>, groupid: i64) -> Result<Vec<UserStatus>, String> {
	sqlx::query_as!(
		UserStatus,
		"
		SELECT username as 'username!', failed_time, failed_msg
		FROM users INNER JOIN group_membership ON users.username = group_membership.user
		WHERE groupid = ?;
		",
		groupid
	)
	.fetch_all(pool)
	.await
	.map_err(|err| format!("Internal database error. {}", err.to_string()))
}

async fn groups_from_user(pool: &Pool<Sqlite>, username: &String) -> Result<Vec<Group>, String> {
	sqlx::query_as!(
		Group,
		"
		SELECT groups.id as 'id!', name, founder
		FROM groups INNER JOIN group_membership
		WHERE group_membership.user = ?;
		",
		username
	)
	.fetch_all(pool)
	.await
	.map_err(|err| format!("Internal database error, {}", err.to_string()))
}