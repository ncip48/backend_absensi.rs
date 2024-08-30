use crate::auth::{decode_jwt, PrivateClaim};
use crate::errors::ApiError;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use serde::Serialize;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Auth;

impl<S, B> Transform<S> for Auth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware { service })
    }
}
pub struct AuthMiddleware<S> {
    service: S,
}

#[derive(Serialize)]
pub struct UnathorizedResponse {
    success: bool,
    msg: &'static str,
}

fn get_bearer_token(req: &ServiceRequest) -> Option<String> {
    req.headers()
        .get(actix_web::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(String::from)
}

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let identity = get_bearer_token(&req).unwrap_or_default();
        // let identity = RequestIdentity::get_identity(&req).unwrap_or("".into());
        let private_claim: Result<PrivateClaim, ApiError> = decode_jwt(&identity);
        let is_logged_in = private_claim.is_ok();
        let unauthorized = !is_logged_in && req.path() != "/api/login";

        if unauthorized {
            let response_body = UnathorizedResponse {
                success: false,
                msg: "Unauthorized",
            };

            let json_body =
                serde_json::to_string(&response_body).unwrap_or_else(|_| "{}".to_string());

            // Create a response with JSON body
            let response = HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(json_body);

            return Box::pin(async move { Ok(req.error_response(response)) });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
