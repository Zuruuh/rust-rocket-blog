use diesel::{Queryable, Selectable, Identifiable};
use uuid::Uuid;

use crate::schema::posts;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
}
