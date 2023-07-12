use std::collections::BTreeMap;

use uuid::Uuid;

use crate::models::{Tag, Post};

pub struct PostsWithTagsDTO(Vec<PostWithTagsDTO>);

#[derive(serde::Serialize)]
pub struct PostWithTagsDTO {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<Tag>
}

impl PostsWithTagsDTO {
    pub fn new(data: Vec<(Post, Tag)>) -> Self {
        let mut results_map = BTreeMap::<Post, Vec<Tag>>::new();
        for (post, tag) in data {
            results_map.entry(post).or_default().push(tag);
        }
        dbg!(results_map);

        Self(vec![])
    }
}
