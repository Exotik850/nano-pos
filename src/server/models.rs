use chrono::NaiveDateTime;
use diesel::prelude::*;



#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::server::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}