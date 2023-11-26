use axum::extract::{Extension, Json};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Deserialize)]
pub struct PostData {
    index: i32,
    field1: String,
    field2: String,
}

#[derive(Serialize)]
pub struct ResponseData {
    message: String,
}

pub async fn update_command(
    client: Extension<Arc<Client>>,
    Json(data): Json<PostData>,
) -> Json<ResponseData> {
    println!(
        "Updating fields at index {} with field1: {} and field2: {}",
        data.index, data.field1, data.field2,
    );

    let response = ResponseData {
        message: format!("Updated fields at index: {}", data.index),
    };

    client
        .execute(
            "UPDATE commands SET command_name = $1, command_response = $2 WHERE id = $3",
            &[&data.field1, &data.field2, &data.index],
        )
        .await
        .unwrap();

    Json(response)
}