use sqlx::{types::Uuid, FromRow, Row};

use super::uuid::uuid_from_row;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: String,
}

impl<'r, R: Row> FromRow<'r, R> for Post
where
    &'r ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    String: ::sqlx::decode::Decode<'r, R::Database>,
    String: ::sqlx::types::Type<R::Database>,
    Uuid: ::sqlx::decode::Decode<'r, R::Database>,
    Uuid: ::sqlx::types::Type<R::Database>,
{
    fn from_row(row: &'r R) -> Result<Self, sqlx::Error> {
        let id = uuid_from_row(&row, "id")?;
        let title = row.try_get("title")?;
        let content: String = row.try_get("content")?;
        let author: String = row.try_get("author")?;

        Ok(Self { id, title, content, author })
    }
}
