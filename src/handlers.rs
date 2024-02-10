use crate::sensor::{self, Sensor};

use axum::{
    http::StatusCode,
    routing::put,
    routing::get,
    routing::delete,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
pub async fn list_sensors() -> Result<Json<Vec<Sensor>>, StatusCode> {
    Ok(Json(vec![
        Sensor { id: 1, name: "Temperature Sensor".into(), location: "Room 101".into(), data: "22°C".into() },
        Sensor { id: 2, name: "Light Sensor".into(), location: "Room 101".into(), data: "500 LUX".into() },
        Sensor { id: 3, name: "Humidity Sensor".into(), location: "Room 101".into(), data: "40 PPMV".into() },
    ]))
}


