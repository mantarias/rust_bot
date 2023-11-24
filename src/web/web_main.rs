use axum::{body::Body, extract::{Query, Path}, http::{Response, StatusCode}, routing::get, routing::post, Router, Json};
use serde_derive::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use axum::response::Html;
use tokio::fs;


#[derive(Deserialize)]
struct PostData {
    field1: String,
    field2: String,
}

#[derive(Serialize)]
struct ResponseData {
    message: String,
}

#[derive(Deserialize)]
struct QueryParams {
    page: Option<String>,
}

pub async fn run_server() {
    let app = Router::new()
        .route("/", get(page_handler))
        .route("/post", post(post_handler))
        .route("/static/*path", get(static_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn page_handler(Query(params): Query<QueryParams>) -> Result<Html<String>, (StatusCode, String)> {
    let page = params.page.unwrap_or_else(|| "index".to_string());
    let path = format!("{}{}", "src/web/www/", page);
    let path = format!("{}{}", path, ".html");
    println!("{}", path);

    serve_html_file(&path).await
}

async fn serve_html_file(path: &str) -> Result<Html<String>, (StatusCode, String)> {
    match tokio::fs::File::open(path).await {
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
    println!("Received field1: {} and {}", data.field1, data.field2);

    let response = ResponseData {
        message: format!("Received field1: {} and {}", data.field1, data.field2),
    };

    Json(response)
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