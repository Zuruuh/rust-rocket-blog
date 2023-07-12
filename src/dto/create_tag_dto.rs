use rocket::{FromForm, serde::Deserialize};

#[derive(Debug)]
#[derive(FromForm, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTagDTO {
    pub label: String,
}
