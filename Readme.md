pub async fn fetch()->Json<Vec<Sensor>>{
    Json(vec![
        Sensor{
            
        }
    ])
}




curl -X PUT http://localhost:3000/sensor \
     -H "Content-Type: application/json" \
     -d '{"id": 1, "name": "Temperature Sensor", "value": 23.5}'







use axum::{extract::Json, response::Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Row};

// Define a struct for the request body
#[derive(Deserialize)]
struct UpdateRequest {
    id: i32,
    title: String,
    content: String,
}

// Define a struct for the response body
#[derive(Serialize)]
struct UpdateResponse {
    message: String,
}

// Define a handler function for the update route
async fn update_note(
    Json(update_request): Json<UpdateRequest>,pool: extract::Extension<PgPool>,) -> Result<Json<UpdateResponse>,StatusCode> {
    // Execute an SQL query to update the note by id
    let rows_affected = sqlx::query("UPDATE notes SET title = $1, content = $2 WHERE id = $3")
        .bind(&update_request.title)
        .bind(&update_request.content)
        .bind(update_request.id)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    // Check if the update was successful
    if rows_affected == 1 {
        // Return a success message
        Ok(Json(UpdateResponse {
            message: "Note updated successfully".to_string(),
        }))
    } else {
        // Return a not found error
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]
async fn main() {
    // Create a connection pool to the database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://username:password@localhost/database")
        .await
        .unwrap();

    // Build the router with the update route
    let app = Router::new().route("/notes", post(update_note)).layer(
        // Add the connection pool as a shared state
        AddExtensionLayer::new(pool),
    );

    // Run the server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
