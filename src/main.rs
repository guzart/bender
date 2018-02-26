#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate serde_derive;

mod users;

#[cfg(test)]
mod tests;

#[get("/")]
fn hello() -> &'static str {
  "Hello, world!"
}

fn main() {
  rocket::ignite()
    .mount("/", routes![hello])
    .mount("/users", users::routes())
    .launch();
}
