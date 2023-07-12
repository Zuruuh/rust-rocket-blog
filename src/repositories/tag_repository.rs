// use async_trait::async_trait;
//
// use crate::{dto::CreateTagDTO, models::Tag, db::BlogConnection, schema::tags::{self, dsl::*}};
//
// #[async_trait]
// pub trait TagRepository {
//     fn save(&mut self, create_tag_dto: CreateTagDTO) -> Tag;
// }
//
// pub struct PersistentTagRepository<'a> {
//     db: &'a mut BlogConnection,
// }
//
// impl<'a> PersistentTagRepository<'a> {
//     pub fn new(db: &'a mut BlogConnection) -> Self {
//         Self { db }
//     }
// }
//
// #[async_trait]
// impl<'a> TagRepository for PersistentTagRepository<'a> {
//     async fn save(&mut self, create_tag_dto: CreateTagDTO) -> Tag {
//         let tag = Tag::new(create_tag_dto.label);
//         self.db.run(move |connection| {
//             insert_into()
//         });
//     } 
// }
