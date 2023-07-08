use rocket::{get, routes, serde::json::Json};

use crate::{
    db::BlogConnection,
    controllers::mapping::RouteMapping,
    models::Post,
    repositories::{PersistentPostRepository, PostRepository},
};

#[get("/?<limit>&<offset>")]
pub async fn list(
    mut db: BlogConnection,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Json<Vec<Post>> {
    let mut post_repository = PersistentPostRepository::new(&mut db);
    let posts = post_repository
        .list(limit.unwrap_or(10), offset.unwrap_or(0))
        .await;

    Json(posts)
}

pub fn routes() -> RouteMapping {
    ("/blog", routes![list])
}
