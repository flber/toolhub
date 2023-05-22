use rocket_dyn_templates::{context, Template};


#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/", $crate::dev:: $($t)*))
}

pub use session_uri as uri;

#[get("/")]
fn index() -> Template {
	Template::render(
		"home",
		context! {},
	)
}

pub fn routes() -> Vec<rocket::Route> {
	routes![index]
}
