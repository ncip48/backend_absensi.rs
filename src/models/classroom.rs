use crate::schema::classrooms;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Serialize)]
pub struct Classroom {
    pub classroom_id: i32,
    pub classroom_name: String,
    pub classroom_status: Option<bool>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Validate)]
#[table_name = "classrooms"]
pub struct NewClassroom {
    #[validate(length(
        min = 3,
        message = "classroom_name is required and must be at least 3 characters"
    ))]
    pub classroom_name: String,
    pub classroom_status: bool,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "classrooms"]
pub struct UpdateClassroom {
    classroom_name: Option<String>,
    classroom_status: Option<bool>,
}
