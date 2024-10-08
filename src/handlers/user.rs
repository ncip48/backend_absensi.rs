use crate::auth::{decode_jwt, PrivateClaim};
use crate::database::establish_connection;
use crate::errors::ApiError;
use crate::models::user::{NewUser, UpdateUser, User};
use crate::validate::validate;
use actix_web::dev::ServiceRequest;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use diesel::prelude::*;

pub async fn get_users() -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    let datas = users
        .load::<User>(&mut connection)
        .expect("Error loading users");
    Ok(HttpResponse::Ok().json(datas))
}

pub async fn create_user(params: web::Json<NewUser>) -> Result<HttpResponse> {
    validate(&params)?;
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();

    let new_user: NewUser = NewUser {
        username: params.username.to_string(),
        name: params.name.to_string(),
        email: params.email.to_string(),
        password: params.password.to_string(),
        role: params.role,
    }
    .into();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut connection)
        .expect("Error inserting new classroom");
    Ok(HttpResponse::Ok().json("data inserted into the database"))
}

pub async fn update_user(
    id: web::Path<i32>,
    user_update: web::Json<UpdateUser>,
) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    let clid = id.into_inner();
    let updated_classroom = diesel::update(users.find(clid))
        .set(&user_update.into_inner())
        .execute(&mut connection)
        .expect("Failed to update user");
    Ok(HttpResponse::Ok().json(updated_classroom))
}

pub async fn delete_user(id: web::Path<i32>) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    let mut connection = establish_connection();
    let clid = id.into_inner();
    diesel::delete(users.find(clid))
        .execute(&mut connection)
        .expect(&format!("Unable to find user {:?}", clid));
    Ok(HttpResponse::Ok().json("Deleted successfully"))
}

fn get_bearer_token(req: HttpRequest) -> Option<String> {
    req.headers()
        .get(actix_web::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(String::from)
}

pub async fn get_profile(req: HttpRequest) -> Result<HttpResponse> {
    use crate::schema::users::dsl::*;
    // Establish database connection
    let mut connection = establish_connection();

    // Extract and decode the JWT token
    let identity = get_bearer_token(req).unwrap_or_default();
    let private_claim: Result<PrivateClaim, ApiError> = decode_jwt(&identity);

    let uid = match private_claim {
        Ok(claim) => claim.user_id, // Adjust this field based on your claim structure
        Err(_) => return Err(ApiError::Unauthorized("Failed".to_string()))?, // Handle decoding errors
    };

    // Fetch user profile from the database
    let user_profile = users
        .filter(user_id.eq(&uid))
        .first::<User>(&mut connection)
        .map_err(|_| ApiError::NotFound("User not found".to_string()))?;

    Ok(HttpResponse::Ok().json(user_profile))
}
