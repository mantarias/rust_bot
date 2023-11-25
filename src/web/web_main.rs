use crate::web::get_commands::get_commands;
use crate::web::page_handler::page_handler;
use crate::web::post_handeler::post_handler;
use axum::{
    body::Body,
    extract::{Extension, Path},
    http::{Response, StatusCode},
    routing::get,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;

use tokio_postgres::Client;

pub async fn run_server(client: Client) {
    // Create table if it doesn't exist
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS commands (command_name text, command_response text)",
            &[],
        )
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(page_handler))
        .route("/post", post(post_handler))
        .route("/get-commands", get(get_commands))
        .route("/static/*path", get(static_handler))
        .layer(Extension(Arc::new(client)));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn static_handler(Path(path): Path<PathBuf>) -> Result<Response<Body>, StatusCode> {
    let path_str = path.to_str().unwrap().trim_start_matches('/'); // Trim the leading '/'
    let file_path = PathBuf::from("src/web/www/static").join(path_str); // Assuming 'static' is your directory
    serve_file(file_path).await
}

async fn serve_file(file_path: PathBuf) -> Result<Response<Body>, StatusCode> {
    match fs::read(file_path).await {
        Ok(contents) => Ok(Response::new(Body::from(contents))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
