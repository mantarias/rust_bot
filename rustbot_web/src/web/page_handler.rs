use axum::response::Html;
use axum::{extract::Query, http::StatusCode};
use serde_derive::Deserialize;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
pub struct QueryParams {
    page: Option<String>,
}

pub async fn page_handler(
    Query(params): Query<QueryParams>,
) -> Result<Html<String>, (StatusCode, String)> {
    let page = params.page.unwrap_or_else(|| "static/index".to_string());
    let path = format!("{}{}", "rustbot_web/src/web/www/", page);
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
