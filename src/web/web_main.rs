use axum::{
    body::Body, extract::{Extension, Json, Path, Query}, http::{Response, StatusCode},
    routing::get, routing::post, Router,
};
use axum::response::Html;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::net::SocketAddr;
use tokio::fs;
use tokio::io::AsyncReadExt;
use tokio_postgres::Client;
use std::sync::Arc;


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
        .route("/static/*path", get(static_handler))
        .layer(Extension(Arc::new(client)));
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
                Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),            }
        }
        Err(e) => Err((StatusCode::NOT_FOUND, e.to_string())),
    }
}

async fn post_handler(client: Extension<Arc<Client>>, Json(data): Json<PostData>) -> Json<ResponseData> {
    println!("Received field1: {} and {}", data.field1, data.field2);

    let response = ResponseData {
        message: format!("Received field1: {} and {}", data.field1, data.field2),
    };

    client
        .execute(
            "INSERT INTO commands (command_name, command_response) VALUES ($1, $2)",
            &[&data.field1,&data.field2],
        )
        .await
        .unwrap();


    let rows = client
        .query("SELECT * FROM hello", &[])
        .await.expect("Failed to run query");

// You can use this to print the value of 'column_name' in the first row
    for row in rows {
        let column_value: String = row.get("column_name");
        println!("{}", column_value);
    }


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