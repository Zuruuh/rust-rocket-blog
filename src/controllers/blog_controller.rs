use rocket::{get, routes};

use crate::controllers::mapping::RouteMapping;


#[get("/")]
pub fn list() -> &'static str {
    "Wsh"
}

pub fn routes() -> RouteMapping {
    ("/blog", routes![list])
}
