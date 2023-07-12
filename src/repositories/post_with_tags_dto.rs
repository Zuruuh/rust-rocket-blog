use std::collections::BTreeMap;

use uuid::Uuid;

use crate::models::{Post, Tag};

#[derive(serde::Serialize)]
pub struct PostsWithTagsDTO(Vec<PostWithTagsDTO>);

#[derive(serde::Serialize)]
pub struct PostWithTagsDTO {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<Tag>,
}

impl PostWithTagsDTO {
    pub fn new(post: Post, tags: Vec<Tag>) -> Self {
        PostWithTagsDTO {
            id: post.id,
            title: post.title,
            content: post.content,
            author: post.author,
            tags,
        }
    }
}

impl PostsWithTagsDTO {
    pub fn new(data: Vec<(Post, Tag)>) -> Self {
        let mut results_map = BTreeMap::<Post, Vec<Tag>>::new();
        for (post, tag) in data {
            results_map.entry(post).or_default().push(tag);
        }

        Self(
            results_map
                .iter()
                .map(|(post, tags)| -> PostWithTagsDTO {
                    PostWithTagsDTO::new(post.to_owned(), tags.to_vec())
                })
                .collect(),
        )
    }
}

impl Default for PostsWithTagsDTO {
    fn default() -> Self {
        Self(vec![])
    }
}
