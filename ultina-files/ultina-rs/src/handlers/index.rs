use crate::files::ultina_page;
use axum::response::Html;

pub async fn get() -> Html<String> {
    let content = ultina_page("index.html")
        .await
        .unwrap_or_else(|e| e.to_string());

    Html(content)
}
