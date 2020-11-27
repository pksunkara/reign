use reign::model::diesel;

diesel::table! {
    articles (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
    }
}
