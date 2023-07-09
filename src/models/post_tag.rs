use uuid::Uuid;
use diesel::{Queryable, Selectable, Associations, Identifiable};

use crate::schema::post_tags;
use super::{Post, Tag};

#[derive(Debug)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Post))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = post_tags)]
#[diesel(primary_key(post_id, tag_id))]
pub struct PostTag {
    pub post_id: Uuid,
    pub tag_id: Uuid,
}
