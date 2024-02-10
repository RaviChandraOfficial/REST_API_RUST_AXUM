use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,serde::Deserialize, serde::Serialize)]
pub struct Sensor {
    pub id: u32,
    pub name: String,
    pub location: String,
    pub data: String,
}


pub struct update_user{
    pub i2:u32,
    pub name:String,
    pub location:String,
    pub data: String,
}