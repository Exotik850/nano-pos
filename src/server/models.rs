use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::server::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::server::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserForm<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub email: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::server::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: i32,
    pub quantity: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::server::schema::product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductForm<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: i32,
    pub quantity: i32,
}