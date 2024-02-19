use axum::{async_trait, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
// use async_trait::async_trait;
use sqlx::FromRow;
// use sqlx::sqlite::SqlitePool;
use sqlx::sqlx_macros::expand_query;



#[derive(Debug, Clone,serde::Deserialize, serde::Serialize)]
pub struct Sensor {
    pub id: u32,
    pub name: String,
    pub location: String,
    pub data: String,
}

#[derive(Debug, Clone,serde::Deserialize, serde::Serialize)]
pub struct post_data{
    pub id:u32,
    pub name: String,
    pub location: String,
    pub data: String,
}



// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    // id:u32,
    // name: String,
    // location :String,
    // data:String,
    pub username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}



