use std::time::UNIX_EPOCH;

use rand::random;
use rocket::{http::CookieJar, serde::{json::Json, Deserialize, Serialize}, State};
use sqlx::{Pool, Sqlite};


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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
	username: String,
}


#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Option<Json<Credentials>>, cookies: &CookieJar<'_>, db: &State<Pool<Sqlite>>)
	-> Json<LoginResponse> {

	let auth = match credentials {
		Some(cred) => authenticate_creds(db, &cred.0).await,
		None => authenticate_session(db, cookies).await.ok_or("No session".to_owned()),
	};

	Json(match auth {
		Err(err) => {
			LoginResponse {
				success: false,
				user: None,
				message: Some(err),
			}
		},
		Ok(u) => {
			update_session(db, cookies, &u).await;
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
		},
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

async fn get_user(pool: &Pool<Sqlite>, username: &String) -> Option<User> {
	let user = sqlx::query_as!(
		User,
		"SELECT username AS 'username!' FROM users WHERE username = ?;",
		username
	).fetch_optional(pool)
		.await
		.unwrap_or(None);

	user
}

async fn add_user(pool: &Pool<Sqlite>, username: &String, password: &String) -> () {
	let _ = sqlx::query!(
		"
		INSERT INTO users (username, password)
		VALUES ($1, $2);
		",
		username, password
	).fetch_one(pool).await;
}

async fn authenticate_session(pool: &Pool<Sqlite>, cookies: &CookieJar<'_>) -> Option<User> {
	let sessionid = cookies
		.get("sessionid")?
		.value()
		.parse::<u32>()
		.ok()?;

	let user = sqlx::query_as!(
		User,
		"SELECT user as 'username!' FROM sessions WHERE id = ?",
		sessionid
	).fetch_optional(pool)
		.await
		.unwrap_or(None);
		
	user
}

async fn authenticate_creds(pool: &Pool<Sqlite>, creds: &Credentials) -> Result<User, String> {
	let user = sqlx::query!(
		"SELECT username as 'username!', password as 'password!' FROM users WHERE username = ?",
		creds.username
	).fetch_optional(pool)
		.await
		.unwrap_or(None);
	
	match user {
		Some(u) => {
			if creds.password == u.password {
				Ok(User { username: u.username })
			} else {
				Err("Password incorrect".to_owned())
			}
		}
		None => {
			Err("Account with this username not found".to_owned())
		}
	}
}

async fn update_session(pool: &Pool<Sqlite>, cookies: &CookieJar<'_>, user: &User) -> () {
	let sessionid = random::<u32>();
	cookies.add(("sessionid", sessionid.to_string()));

	let time = UNIX_EPOCH.elapsed().unwrap_or_default().as_secs_f64();

	let _ = sqlx::query!(
		"
		INSERT INTO sessions (id, user, time)
		VALUES ($1, $2, $3);
		",
		sessionid, user.username, time
	).fetch_one(pool).await;
}
