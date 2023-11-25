use axum::extract::{Extension, Json};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct PostData {
    field1: String,
    field2: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    message: String,
}

pub async fn post_handler(
    client: Extension<Arc<Client>>,
    Json(data): Json<PostData>,
) -> Json<ResponseData> {
    println!("Received field1: {} and {}", data.field1, data.field2);

    let response = ResponseData {
        message: format!("Received field1: {} and {}", data.field1, data.field2),
    };

    client
        .execute(
            "INSERT INTO commands (command_name, command_response) VALUES ($1, $2)",
            &[&data.field1, &data.field2],
        )
        .await
        .unwrap();

    Json(response)
}
