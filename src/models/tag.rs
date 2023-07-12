use diesel::{Queryable, Selectable, Identifiable, Insertable};
use uuid::Uuid;

use crate::schema::tags;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Queryable, Selectable, Identifiable, Insertable)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tag {
    pub id: Uuid,
    pub label: String,
}

impl Tag {
    pub fn new(label: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            label,
        }
    }
}
