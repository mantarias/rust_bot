//! This file is the main entry point for the bot. It sets up the bot and starts it.
//! Will print out a message if updated by a user.
use tokio::fs as tokio_fs;

use crate::web::get_commands::get_commands;
use crate::web::page_handler::page_handler;
use crate::web::post_handler::post_handler;
use crate::web::update_command::update_command;

use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{Response, StatusCode},
    routing::get,
    routing::post,
    routing::put,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;




mod web;

use tokio_postgres::NoTls;


#[tokio::main]
async fn main() {
    // Connect to the database.
    let (db_client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 dbname=rustbot password=Bean1! user=postgres",
        NoTls,
    )
    .await
    .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Start the web server in a separate async task
    db_client
        .execute(
            "CREATE TABLE IF NOT EXISTS commands (command_name text, command_response text)",
            &[],
        )
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(page_handler))
        .route("/create-command", post(post_handler))
        .route("/get-commands", get(get_commands))
        .route("/static/*path", get(static_handler))
        .route("/update-command", put(update_command))
        .layer(Extension(Arc::new(db_client)));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn static_handler(Path(path): Path<PathBuf>) -> Result<Response<Body>, StatusCode> {
    let path_str = path.to_str().unwrap().trim_start_matches('/'); // Trim the leading '/'
    let file_path = PathBuf::from("rustbot_web/src/web/www/static").join(path_str); // Assuming 'static' is your directory
    serve_file(file_path).await
}

async fn serve_file(file_path: PathBuf) -> Result<Response<Body>, StatusCode> {
    match tokio_fs::read(file_path).await {
        Ok(contents) => Ok(Response::new(Body::from(contents))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}