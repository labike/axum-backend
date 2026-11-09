use axum::{Extension, http::{HeaderMap, header}, response::IntoResponse};
use axum_extra::extract::cookie::Cookie;
use chrono::Duration;
use tower_http::classify::ServerErrorsFailureClass::StatusCode;

use crate::{AppState, dto::{LoginUserDto, RegisterUserDto, UserLoginResponseDto}, error::{ErrorMessage, HttpError}, mail::mails::send_verification_email, util::{password, token}};

pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(
        |e| HttpError::bad_request(e.to_string())
    );

    let verification_token = uuid::uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);
    let hash_password = password::hash(&body.password)
      .map_err(|e| HttpError::server_error(e.to_string()));
    let result = app_state.db_client.save_user(
        &body.name,
        &body.email,
        &hash_password,
        &verification_token,
        expires_at,
    ).await;

    match result {
        Ok(_user) => {
            let send_email_result = send_verification_email(&body.email, &body.name, &verification_token);
            if let Err(e) = send_email_result {
                eprintln!("Fail to send verification email: {}", e);
            }
            Ok((StatusCode::CREATED, Json(Response {
                status: "success",
                message: "Register successfully!"
            })))
        },
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(HttpError::unique_constraint_violation(ErrorMessage::EmailExists.to_string()))
            } else {
                Err(HttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(HttpError::server_error(e.to_string()))
    }
}

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;

    let result = app_state.db_client.get_user(None, None, Some(&body.email), None).await.map_err(
        |e| HttpError::server_error(e.to_string())
    )?;

    let user = result.ok_err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()))?;
    let password_matched = password::compare(&body.password, &user.password).map_err(
        |_| HttpError::bad_request(ErrorMessage::WrongCredentials.to_string())
    )?;

    if password_matched {
        let token = token::create_token(
            &user.id,
            &app_state.env.jwt_secret_key.as_bytes(),
            app_state.env.jwt_expiration
        ).map_err(|e| HttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_expiration * 60);
        let cookie = Cookie::build(("token", token.clone()))
          .path("/")
          .max_age(cookie_duration)
          .http_only(true)
          .build();

        let response = axum::response::Json(UserLoginResponseDto {
            status: "success".to_string(),
            token,
        });
        let mut header = HeaderMap::new();
        header.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        let mut response = response.into_response();
        response.headers_mut().extend(headers);

        Ok(response)
    } else {
        Err(HttpError::bad_request(ErrorMessage::WrongCredentials.to_string()));
    }
}