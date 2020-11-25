diesel::table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
    }
}
