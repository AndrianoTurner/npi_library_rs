// @generated automatically by Diesel CLI.

diesel::table! {
    user_table (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        first_name -> Nullable<Varchar>,
        second_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
    }
}
