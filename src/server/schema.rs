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
        #[max_length = 3000]
        message -> Nullable<Varchar>,
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
    estimate (id) {
        id -> Int4,
        customer_id -> Nullable<Int4>,
        user_id -> Int4,
        #[max_length = 255]
        doc_number -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    estimate_product (id) {
        id -> Int4,
        estimate_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price -> Nullable<Int4>,
        tax_rate -> Nullable<Float8>,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    invoice (id) {
        id -> Int4,
        customer_id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        doc_number -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
        order_date -> Timestamp,
        delivery_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    invoice_product (id) {
        id -> Int4,
        invoice_id -> Int4,
        product_id -> Int4,
        quantity -> Int4,
        price -> Nullable<Int4>,
        tax_rate -> Nullable<Float8>,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        sku -> Varchar,
        #[max_length = 3000]
        description -> Nullable<Varchar>,
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

diesel::joinable!(estimate -> contact (customer_id));
diesel::joinable!(estimate -> users (user_id));
diesel::joinable!(estimate_product -> estimate (estimate_id));
diesel::joinable!(estimate_product -> product (product_id));
diesel::joinable!(invoice -> contact (customer_id));
diesel::joinable!(invoice -> users (user_id));
diesel::joinable!(invoice_product -> invoice (invoice_id));
diesel::joinable!(invoice_product -> product (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    contact,
    estimate,
    estimate_product,
    invoice,
    invoice_product,
    product,
    users,
);
