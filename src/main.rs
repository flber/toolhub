#[macro_use]
extern crate rocket;
// use rocket::request;
// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;
use rocket::fs::FileServer;

pub mod qr;
mod home;
// mod dev;

// use api::count::{self, UserCount};

#[launch]
fn rocket() -> _ {
	rocket::build()
		// .manage(UserCount::default())
		// .manage(UserIPs(vec![]))
		.attach(Template::fairing())
		.mount("/", FileServer::from("static/"))
		.mount("/", home::routes())
}