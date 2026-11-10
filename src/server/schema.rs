// @generated automatically by Diesel CLI.

diesel::table! {
    contact (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 20]
        phone_number -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        message -> Nullable<Text>,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 20]
        zip_code -> Varchar,
        #[max_length = 100]
        country -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        sku -> Varchar,
        description -> Nullable<Text>,
        price -> Int4,
        quantity -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(contact, product, users,);
