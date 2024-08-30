use crate::database::establish_connection;
use crate::models::classroom::{Classroom, NewClassroom, UpdateClassroom};
use crate::validate::validate;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

pub async fn get_classrooms() -> Result<HttpResponse> {
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();
    let datas = classrooms
        .load::<Classroom>(&mut connection)
        .expect("Error loading classrooms");
    Ok(HttpResponse::Ok().json(datas))
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

    diesel::insert_into(classrooms)
        .values(&new_classroom)
        .execute(&mut connection)
        .expect("Error inserting new classroom");
    Ok(HttpResponse::Ok().json("data inserted into the database"))
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
    let updated_classroom = diesel::update(classrooms.find(clid))
        .set(&classroom_update.into_inner())
        .execute(&mut connection)
        .expect("Failed to update student");
    Ok(HttpResponse::Ok().json(updated_classroom))
}

pub async fn delete_classroom(id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::classrooms::dsl::*;
    let mut connection = establish_connection();
    let clid = id.into_inner();
    diesel::delete(classrooms.find(clid))
        .execute(&mut connection)
        .expect(&format!("Unable to find classroom {:?}", clid));
    Ok(HttpResponse::Ok().json("Deleted successfully"))
}
