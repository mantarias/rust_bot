use axum::extract::{Extension, Json};
use serde_derive::Serialize;
use std::sync::Arc;
use tokio_postgres::Client;

#[derive(Serialize)]
struct CommandData {
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
    for row in rows {
        let command = row.get(0);
        let response = row.get(1);
        commands.push(CommandData { command, response });
    }

    Json(ResponseCommandData { commands })
}
