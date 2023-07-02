use async_trait::async_trait;

use crate::{models::Post, db::BlogConnection};

#[async_trait]
pub trait PostRepository {
    async fn list(&self, limit: usize, offset: usize) -> Option<Vec<Post>>;
}

pub struct PersistentPostRepository<'a> {
    db: &'a mut BlogConnection,
}

impl<'a> PersistentPostRepository<'a> {
    pub fn new(db: &'a mut BlogConnection) -> Self {
        Self { db: db }
    }
}

#[async_trait]
impl<'a> PostRepository for PersistentPostRepository<'a> {
    async fn list(&self, limit: usize, offset: usize) -> Option<Vec<Post>> {
        let mut connection = *self.db;
        let maybe_posts = sqlx::query_as::<_ ,Post>("SELECT * FROM posts LIMIT 10")
            .fetch_all(&connection)
            .await;

        if maybe_posts.is_err() {
            return None;
        }

        Some(maybe_posts.unwrap())
    }
}
