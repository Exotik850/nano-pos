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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    invoice (id) {
        id -> Int4,
        customer_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        doc_number -> Varchar,
        description -> Nullable<Text>,
        order_date -> Timestamp,
        delivery_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    invoice_product (invoice_id, product_id) {
        invoice_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price -> Nullable<Int4>,
        tax_rate -> Nullable<Float8>,
        description -> Nullable<Text>,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(invoice -> contact (customer_id));
diesel::joinable!(invoice -> users (user_id));
diesel::joinable!(invoice_product -> invoice (invoice_id));
diesel::joinable!(invoice_product -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(contact, invoice, invoice_product, product, users,);
