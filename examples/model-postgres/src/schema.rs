use reign::model::diesel;

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}
