#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "🚀 Welcome to Rust Web API!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("👋 Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
