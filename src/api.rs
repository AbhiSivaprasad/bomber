use mongodb::Database;
use serde::Deserialize;
use std::sync::Arc;
use warp::http::StatusCode;

use crate::user::{User, UserError};

#[derive(Deserialize)]
pub struct UserParams {
    username: String,
    password: String,
}

pub async fn register(
    params: UserParams,
    db: Arc<Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: check for existing username
    let user = match User::new(&params.username, &params.password) {
        Ok(user) => user,
        _ => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };
    if user.save(&db).is_err() {
        return Ok(StatusCode::INTERNAL_SERVER_ERROR);
    }
    Ok(StatusCode::OK)
}

pub async fn login(
    params: UserParams,
    db: Arc<Database>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _user = match User::login(&db, &params.username, &params.password) {
        Ok(user) => user,
        Err(UserError::NotFound) => return Ok(StatusCode::UNAUTHORIZED),
        _ => return Ok(StatusCode::INTERNAL_SERVER_ERROR),
    };
    Ok(StatusCode::OK)
}

pub async fn logout() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(StatusCode::OK)
}
