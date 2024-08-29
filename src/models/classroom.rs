use crate::schema::classrooms;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Classroom {
    pub classroom_id: i32,
    pub classroom_name: String,
    pub classroom_status: Option<bool>,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "classrooms"]
pub struct NewClassroom {
    pub classroom_name: String,
    pub classroom_status: bool,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "classrooms"]
pub struct UpdateClassroom {
    classroom_name: Option<String>,
    classroom_status: Option<bool>,
}
