use async_trait::async_trait;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db::BlogConnection,
    models::{Post, Tag},
    repositories::PostsWithTagsDTO,
    schema::{post_tags, posts, tags},
};

#[async_trait]
pub trait PostRepository {
    async fn list(&mut self, limit: i64, offset: i64) -> PostsWithTagsDTO;
}

pub struct PersistentPostRepository<'a> {
    db: &'a mut BlogConnection,
}

impl<'a> PersistentPostRepository<'a> {
    pub fn new(db: &'a mut BlogConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> PostRepository for PersistentPostRepository<'a> {
    async fn list(&mut self, limit: i64, offset: i64) -> PostsWithTagsDTO {
        let maybe_posts = self
            .db
            .run(move |connection| {
                posts::table
                    .inner_join(post_tags::table.inner_join(tags::table))
                    .select((Post::as_select(), Tag::as_select()))
                    .limit(limit)
                    .offset(offset)
                    .load::<(Post, Tag)>(connection)
            })
            .await;

        if maybe_posts.is_err() {
            println!(
                "An error occured while loading posts: {:?}",
                maybe_posts.unwrap_err()
            );
            return PostsWithTagsDTO::default();
        }

        PostsWithTagsDTO::new(maybe_posts.unwrap())
    }
}
