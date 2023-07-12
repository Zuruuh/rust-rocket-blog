use std::collections::HashMap;

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
        let mut tags_map: HashMap<String, Vec<&Tag>> = HashMap::new();
        let mut posts_map: HashMap<&String, &Post> = HashMap::new();

        data.iter().for_each(|(post, tag)| {
            let post_id = post.id.to_string();
            posts_map.insert(&post_id, post);
            if tags_map.contains_key(&post_id) {
                let mut new_tags = tags_map.remove(&post_id).unwrap();
                new_tags.push(&tag);
                tags_map.insert(post_id, new_tags.to_vec());
            } else {
                tags_map.insert(post_id, vec![tag]);
            }
        });
        dbg!(tags_map);

        Self(vec![])
    }
}
