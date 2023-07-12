use diesel::{Queryable, Selectable, Identifiable};
use uuid::Uuid;

use crate::schema::tags;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub label: String,
}
