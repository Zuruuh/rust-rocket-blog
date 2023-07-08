use std::ops::DerefMut;

use async_trait::async_trait;

use crate::{models::Post, db::BlogConnection};

#[async_trait]
pub trait PostRepository {
    async fn list(&mut self, limit: i32, offset: i32) -> Option<Vec<Post>>;
}

pub struct PersistentPostRepository<'a> {
    // db: Box<&'a BlogConnection>,
    db: &'a mut BlogConnection,
}

impl<'a> PersistentPostRepository<'a> {
    pub fn new(db: &'a mut BlogConnection) -> Self {
        // Self { db: Box::new(db) }
        Self { db }
    }
}

#[async_trait]
impl<'a> PostRepository for PersistentPostRepository<'a> {
    async fn list(&mut self, limit: i32, offset: i32) -> Option<Vec<Post>> {
        let maybe_posts = sqlx::query_file_as!(Post, "queries/post/list.sql", limit, offset)
            .fetch_all(self.db.deref_mut())
            .await;

        if maybe_posts.is_err() {
            return None;
        }

        Some(maybe_posts.unwrap())
    }
}
