use rocket::{get, routes, serde::json::Json};

use crate::{controllers::mapping::RouteMapping, db::BlogConnection, repositories::{PostRepository, PersistentPostRepository}, models::Post};


#[get("/")]
pub async fn list(mut db: BlogConnection) -> Json<Vec<Post>> {
    let mut post_repository = PersistentPostRepository::new(&mut db);
    let posts = post_repository.list(10, 0).await;
    dbg!(&posts);
    let my = &mut *db;
    sqlx::query("").fetch_all(my);

    Json(posts.unwrap_or(vec![]))
}

pub fn routes() -> RouteMapping {
    ("/blog", routes![list])
}
