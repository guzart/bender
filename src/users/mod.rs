extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate uuid;

use rocket::response::content;
use self::uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct User {
  id: Uuid,
}

#[post("/")]
fn new() -> content::Json<String> {
  let user = User {
    id: Uuid::new_v4()
  };
  let json = serde_json::to_string(&user).expect("Fail to serialize");
  content::Json(json)
}

#[put("/<id>")]
fn update(id: String) -> content::Json<&'static str> {
  print!("{}", id);
  content::Json("{ 'id': '' }")
}

pub fn routes() -> Vec<rocket::Route> {
  routes![new, update]
}
