use reign::model::diesel;

diesel::table! {
    articles (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        text -> Text,
        user_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        description -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(comments -> articles (article_id));
diesel::joinable!(comments -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    users,
);
