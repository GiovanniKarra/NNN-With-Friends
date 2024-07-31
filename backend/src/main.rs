use rocket::{fs::FileServer, http::{Cookie, CookieJar}};

#[macro_use] extern crate rocket;

// #[get("/")]
// fn yo(cookie: Cookie) -> Option<String> {
// 	jar.get("message").map(|crumb| format!("Message: {}", crumb.value()))
// }

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", FileServer::from("../frontend/dist/"))
}
