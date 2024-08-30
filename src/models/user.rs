use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: i8,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "users"]
pub struct NewUser {
    #[validate(length(min = 1, message = "username is required"))]
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: i8,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: i8,
}
