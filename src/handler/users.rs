use std::sync::Arc;

use axum::{Extension, response::IntoResponse};

use crate::{AppState, dto::{FilterUserDto, UserData, UserListResponseDto, UserResponseDto}, error::HttpError};

pub async fn get_me(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JwtAuthMiddleware>,
) -> Response<impl IntoResponse, HttpError> {
    let filter_user = FilterUserDto::filter_user(&user.user);
    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData {
            user: filter_user
        }
    };

    Ok(Json(response_data))
}

pub async fn get_under_user(
  Query(query_params): Query<RequestQueryDto>,
  Extension(app_state): Extension<AppState>,
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state.db_client
      .get_users(page as u32, limit)
      .await
      .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = app_state.db_client.get_user_count().await.map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        result: user_count,
    };

    Ok(Json(Response))
}