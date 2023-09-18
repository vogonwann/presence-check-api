use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub created_at: String
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub last_name: String
}