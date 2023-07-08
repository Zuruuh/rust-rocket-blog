use sqlx::{Row, types::uuid::Uuid};

pub fn uuid_from_row<'r, R: Row>(row: &'r R, column: &'r str) -> Result<Uuid, sqlx::Error> 
where
    &'r ::std::primitive::str: ::sqlx::ColumnIndex<R>,
    Uuid: ::sqlx::decode::Decode<'r, R::Database>,
    Uuid: ::sqlx::types::Type<R::Database>,
{
    let t = row.try_get(column);

    t
}
