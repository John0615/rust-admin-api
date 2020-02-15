table! {
    users (id) {
        id -> Int4,
        salt -> Varchar,
        password_digest -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        role -> Varchar,
        login_name -> Varchar,
        status -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
