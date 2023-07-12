use rocket::{FromForm, serde::Deserialize};

use super::CreateTagDTO;

#[derive(Debug)]
#[derive(FromForm, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatePostDTO {
    pub title: String,
    pub content: String,
    pub author: String,
    // pub tags: Vec<CreateTagDTO>,
}
