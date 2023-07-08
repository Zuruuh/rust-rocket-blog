use rocket::{get, routes, serde::json::Json};

use crate::{controllers::mapping::RouteMapping, db::BlogConnection, repositories::{PostRepository, PersistentPostRepository}, models::Post};

#[get("/?<limit>&<offset>")]
pub async fn list(mut db: BlogConnection, limit: Option<i32>, offset: Option<i32>) -> Json<Vec<Post>> {
    let mut post_repository = PersistentPostRepository::new(&mut db);
    let posts = post_repository.list(limit.unwrap_or(10), offset.unwrap_or(0)).await;

    Json(posts.unwrap_or(vec![]))
}

pub fn routes() -> RouteMapping {
    ("/blog", routes![list])
}
