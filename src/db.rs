use rocket_db_pools::{Connection, Database, sqlx::PgPool};

#[derive(Database)]
#[database("blog")]
pub struct Blog(PgPool);

pub type BlogConnection = Connection<Blog>;
