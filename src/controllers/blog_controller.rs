use rocket::{get, routes, serde::json::Json};

use crate::{
    db::BlogConnection,
    controllers::mapping::RouteMapping,
    repositories::{PersistentPostRepository, PostRepository, PostsWithTagsDTO},
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

pub fn routes() -> RouteMapping {
    ("/blog", routes![list_posts_with_tags])
}
