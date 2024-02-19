use crate::sensor::{self, Sensor, post_data,User,CreateUser};

use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::{
    http::StatusCode,
    Json,
};
use axum::{extract::State};
use sqlx::postgres::{PgPool, PgPoolOptions};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;


use serde::{Deserialize, Serialize};

use sqlx::Postgres;

pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select * from sensor_list  ")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}


pub async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select * from sensor_var_char ")
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)
}


pub struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}



/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}


pub async fn put_user(Json(payload): Json<post_data>)  ->  Json<post_data>{
    // handle the PUT request here
    // for example, update an item in a database
    // println!("Updating item: {:?}", payload);
    let user= post_data{
        id:payload.id,
        name:payload.name,
        location:payload.location,
        data:payload.data,
    };
    // Respond with a status code and a message
    Json(user)
}


// this argument tells axum to parse the request body
// as JSON into a `CreateUser` type
pub async fn create_user(Json(payload): Json<CreateUser>,) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 3,
        username: payload.username,
    };
    
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
    