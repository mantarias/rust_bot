use axum::{extract::{Query}, routing::get, Router, response::Html, http::StatusCode, Json, http};
use std::net::SocketAddr;
use axum::routing::post;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize)]
struct PostData {
    field1: String,
    field2: String
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
    // Include any other data you want to send back
}

#[derive(Deserialize)]
struct QueryParams {
    page: String,
}

pub async fn run_server() {
    let app = Router::new()
        .route("/", get(page_handler))
        .route("/post", post(post_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn page_handler(Query(params): Query<QueryParams>) -> Result<Html<String>, (StatusCode, String)> {
    let path = format!("{}{}", "src/web/www/", params.page);
    let path = format!("{}{}", path, ".html");
    println!("{}", path);

    serve_html_file(&path).await
}

// Utility function to serve HTML files
async fn serve_html_file(path: &str) -> Result<Html<String>, (StatusCode, String)> {
    match File::open(path).await {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents).await {
                Ok(_) => Ok(Html(contents)),
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
            }
        }
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn post_handler(Json(data): Json<PostData>) -> Json<ResponseData> {
    println!("Received field1: {} and {}", data.field1,data.field2);

    // Create your response data
    let response = ResponseData {
        message: format!("Received field1: {} and {}", data.field1,data.field2)
        // Set other fields as needed
    };

    // Return JSON response
    Json(response)
}


