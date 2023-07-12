use rocket::{get, routes, serde::json::Json, post, form::Form};

use crate::{
    db::BlogConnection,
    controllers::mapping::RouteMapping,
    repositories::{PersistentPostRepository, PostRepository, PostsWithTagsDTO}, dto::CreatePostDTO,
};

#[get("/?<limit>&<offset>")]
pub async fn list_posts_with_tags(
    mut db: BlogConnection,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Json<PostsWithTagsDTO> {
    let mut post_repository = PersistentPostRepository::new(&mut db);
    let posts = post_repository
        .list(limit.unwrap_or(10), offset.unwrap_or(0))
        .await;

    Json(posts)
}

#[post("/", format = "application/json", data = "<create_post_dto>")]
pub async fn create_post(mut db: BlogConnection, create_post_dto: Json<CreatePostDTO>) -> &'static str {
    dbg!(create_post_dto);
    db.run

    "yo"
}

pub fn routes() -> RouteMapping {
    ("/blog", routes![list_posts_with_tags, create_post])
}
