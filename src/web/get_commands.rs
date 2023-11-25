use axum::extract::{Extension, Json};
use serde_derive::Serialize;
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Serialize)]
struct CommandData {
    index: i32,
    command: String,
    response: String,
}

#[derive(Serialize)]
pub struct ResponseCommandData {
    commands: Vec<CommandData>,
}

pub async fn get_commands(client: Extension<Arc<Client>>) -> Json<ResponseCommandData> {
    let rows = client
        .query("SELECT * FROM commands", &[])
        .await
        .expect("Failed to run query");

    let mut commands = Vec::new();
    for (index, row) in rows.into_iter().enumerate() {
        let command = row.get(1);
        let response = row.get(2);
        commands.push(CommandData { index: index as i32, command, response });
    }

    Json(ResponseCommandData { commands })
}