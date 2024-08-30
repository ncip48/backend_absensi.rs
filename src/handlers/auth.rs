use crate::auth::{create_jwt, hash, PrivateClaim};
use crate::database::establish_connection;
use crate::errors::ApiError;
use crate::models::user::User;
use crate::validate::validate;
use actix_identity::Identity;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use diesel::ExpressionMethods;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "username is required"))]
    pub username: String,

    #[validate(length(
        min = 8,
        message = "password is required and must be at least 8 characters"
    ))]
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    success: bool,
    msg: &'static str,
    data: Auth,
}
#[derive(Serialize, Deserialize)]
pub struct Auth {
    user_id: i32,
    username: String,
    email: String,
    token: Option<String>, // Add an optional JWT field
}

/// Login a user
/// Create and remember their JWT
pub async fn login(id: Identity, params: web::Json<LoginRequest>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    use crate::schema::users::dsl::*;
    validate(&params)?;

    // Validate that the username + hashed password matches
    let hashed = hash(&params.password);
    let user = users
        .filter(username.eq(&params.username.to_string()))
        .filter(password.eq(&hashed.to_string()))
        .first::<User>(&mut connection)
        .map_err(|_| ApiError::NotFound("User not found".to_string()))?;

    // Create a JWT token for the user
    let private_claim = PrivateClaim::new(user.user_id, user.username.clone());
    let jwt = create_jwt(private_claim)?;

    // Remember the token in the user's identity
    id.remember(jwt.clone());

    // Prepare the response with the JWT
    let response = AuthResponse {
        success: true,
        msg: "Login success",
        data: Auth {
            user_id: user.user_id,
            email: user.email,
            username: user.username,
            token: Some(jwt),
        },
    };

    // Return the user information as JSON
    Ok(HttpResponse::Ok().json(response))
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity) -> Result<HttpResponse> {
    id.forget();
    Ok(HttpResponse::Ok().json("Ok"))
}
