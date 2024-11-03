use std::time::UNIX_EPOCH;

use rand::random;
use rocket::{
	http::CookieJar,
	serde::{json::Json, Deserialize, Serialize},
	State,
};
use sqlx::{Pool, Sqlite};
use sha256::digest;

use crate::users::{User, get_user};


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginResponse {
	success: bool,
	message: Option<String>,
	user: Option<User>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Credentials {
	username: String,
	password: String,
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Option<Json<Credentials>>, cookies: &CookieJar<'_>, db: &State<Pool<Sqlite>>)
	-> Json<LoginResponse> {

	let mut new_session = true;
	let auth = match credentials {
		Some(cred) => authenticate_creds(db, &cred.0).await,
		None => {
			new_session = false;
			authenticate_session(db, cookies).await
		},
	};

	Json(match auth {
		Err(err) => LoginResponse {
			success: false,
			user: None,
			message: Some(err),
		},
		Ok(u) => {
			if new_session { add_session(db, cookies, &u).await; }
			else { update_session(db, cookies).await }
			LoginResponse {
				success: true,
				user: Some(u),
				message: Some("login successful".to_owned()),
			}
		}
	})
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn signup(credentials: Json<Credentials>, db: &State<Pool<Sqlite>>) -> Json<LoginResponse> {
	let mut success = false;
	let message: &str;
	match get_user(db, &credentials.username).await {
		Some(_) => {
			message = "account with this username already exists";
		}
		None => {
			add_user(db, &credentials.username, &credentials.password).await;
			success = true;
			message = "created account successfully";
		}
	};

	Json(LoginResponse {
		success: success,
		message: Some(message.to_owned()),
		user: None,
	})
}

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>, db: &State<Pool<Sqlite>>) -> Result<String, String> {
	let sessionid = get_sessionid(cookies)
		.ok_or("couldn't parse sessionid")?;

	sqlx::query!(
		"DELETE FROM sessions WHERE id = ?",
		sessionid
	)
	.execute(db.inner())
	.await
	.map_err(|_err| "database deletion error")?;

	Ok("Logged out successfully".to_owned())
}

async fn add_user(pool: &Pool<Sqlite>, username: &String, password: &String) -> () {
	let hashed_password = digest(password);
	let _ = sqlx::query!(
		"
		INSERT INTO users (username, password)
		VALUES ($1, $2);
		",
		username, hashed_password
	)
	.execute(pool)
	.await;
}

pub async fn authenticate_session(pool: &Pool<Sqlite>, cookies: &CookieJar<'_>) -> Result<User, String> {
	let sessionid = get_sessionid(cookies);

	sqlx::query_as!(
		User,
		"SELECT user as 'username!' FROM sessions WHERE id = ?",
		sessionid
	)
	.fetch_optional(pool)
	.await
	.map_err(|err| format!("Couldn't authenticate user. {}", err.to_string()))
	.map(|u| u.ok_or("Couldn't authenticate user. Session not found".to_owned()))?  // is this genius or just unreadable??
}

async fn authenticate_creds(pool: &Pool<Sqlite>, creds: &Credentials) -> Result<User, String> {
	let user = sqlx::query!(
		"
		SELECT username as 'username!', password as 'password!'
		FROM users WHERE username = ?
		",
		creds.username
	)
	.fetch_optional(pool)
	.await
	.unwrap_or(None);

	match user {
		Some(u) => {
			if digest(&creds.password) == u.password {
				Ok(User { username: u.username })
			} else {
				Err("Password incorrect".to_owned())
			}
		}
		None => Err("Account with this username not found".to_owned()),
	}
}

async fn add_session(pool: &Pool<Sqlite>, cookies: &CookieJar<'_>, user: &User) -> () {
	let sessionid = random::<i64>();
	cookies.add(("sessionid", sessionid.to_string()));

	let time = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs() as i64;

	let _ = sqlx::query!(
		"
		INSERT INTO sessions (id, user, time)
		VALUES ($1, $2, $3);
		",
		sessionid, user.username, time
	)
	.fetch_one(pool)
	.await;
}

async fn update_session(pool: &Pool<Sqlite>, cookies: &CookieJar<'_>) -> () {
	let sessionid = get_sessionid(cookies);
	let time = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs() as i64;

	let _ = sqlx::query!(
		"
		UPDATE sessions
		SET time = ?
		WHERE id = ?
		",
		time, sessionid
	)
	.fetch_one(pool)
	.await;
}

pub fn get_sessionid(cookies: &CookieJar<'_>) -> Option<i64> {
	cookies
		.get("sessionid")?
		.value()
		.parse::<i64>()
		.ok()
}