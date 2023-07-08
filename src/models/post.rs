use diesel::{Queryable, Selectable};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
}
