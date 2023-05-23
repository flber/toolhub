use rocket_dyn_templates::{context, Template};

use crate::qr;

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::dev:: $($t)*))
}

pub use session_uri as uri;

#[get("/")]
fn index() -> Template {
	let mut errors = String::new();
	match qr::encode("static/test.txt", "static/test.png") {
		Ok(_) => (),
		Err(e) => {
			errors.push_str(&format!("{:?}", e));
		}
	}

	Template::render(
		"home",
		context! {
			errors: errors,
			output: qr::decode("static/test.png"),
		},
	)
}

pub fn routes() -> Vec<rocket::Route> {
	routes![index]
}
