use crate::database::establish_connection;
use crate::models::classroom::{Classroom, NewClassroom, UpdateClassroom};
use crate::validate::validate;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde::Serialize;
#[derive(Serialize)]
pub struct ClassroomResponse {
    success: bool,
    msg: &'static str,
    data: Vec<Classroom>,
}

pub async fn get_classrooms() -> Result<HttpResponse> {
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();
    let datas = match classrooms.load::<Classroom>(&mut connection) {
        Ok(data) => data,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(ClassroomResponse {
                success: false,
                msg: "Error loading classrooms",
                data: vec![],
            }));
        }
    };

    let response = ClassroomResponse {
        success: true,                    // You can adjust this based on your actual logic
        msg: "Data fetched successfully", // A message to include in the response
        data: datas,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn create_classroom(params: web::Json<NewClassroom>) -> Result<HttpResponse> {
    validate(&params)?;
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();

    let new_classroom: NewClassroom = NewClassroom {
        classroom_name: params.classroom_name.to_string(),
        classroom_status: params.classroom_status,
    }
    .into();

    match diesel::insert_into(classrooms)
        .values(&new_classroom)
        .execute(&mut connection)
    {
        Ok(_) => {
            let response = ClassroomResponse {
                success: true,
                msg: "Classroom created successfully",
                data: vec![],
            };

            return Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            let response = ClassroomResponse {
                success: false,
                msg: "Error creating classroom",
                data: vec![],
            };

            return Ok(HttpResponse::InternalServerError().json(response));
        }
    }
}

pub async fn update_classroom(
    id: web::Path<i32>,
    classroom_update: web::Json<UpdateClassroom>,
) -> Result<HttpResponse> {
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();
    let clid = id.into_inner();
    // Use the `update` method of the Diesel ORM
    //to update the student's record
    match diesel::update(classrooms.find(clid))
        .set(&classroom_update.into_inner())
        .execute(&mut connection)
    {
        Ok(_) => {
            let response = ClassroomResponse {
                success: true,
                msg: "Classroom updated successfully",
                data: vec![],
            };

            return Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            let response = ClassroomResponse {
                success: false,
                msg: "Error updating classroom",
                data: vec![],
            };

            return Ok(HttpResponse::InternalServerError().json(response));
        }
    }
}

pub async fn delete_classroom(id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();
    let clid = id.into_inner();
    match diesel::delete(classrooms.find(clid)).execute(&mut connection) {
        Ok(_) => {
            let response = ClassroomResponse {
                success: true,
                msg: "Classroom deleted successfully",
                data: vec![],
            };

            return Ok(HttpResponse::Ok().json(response));
        }
        Err(e) => {
            let response = ClassroomResponse {
                success: false,
                msg: "Error deleting classroom",
                data: vec![],
            };

            return Ok(HttpResponse::InternalServerError().json(response));
        }
    }
}
