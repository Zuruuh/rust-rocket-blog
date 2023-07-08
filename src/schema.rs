// @generated automatically by Diesel CLI.

diesel::table! {
    post_tags (post_id, tag_id) {
        post_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 255]
        author -> Varchar,
    }
}

diesel::table! {
    tags (id) {
        id -> Uuid,
        #[max_length = 32]
        label -> Varchar,
    }
}

diesel::joinable!(post_tags -> posts (post_id));
diesel::joinable!(post_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    post_tags,
    posts,
    tags,
);
