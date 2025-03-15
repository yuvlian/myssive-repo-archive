use std::path::Path;

pub const ASSETS_DIR: &str = "ultina-page/assets";
pub const ULTINA_PAGE_DIR: &str = "./ultina-page";
pub const HTMX_RSP_DIR: &str = "./ultina-page/htmx-rsp";

pub async fn ultina_page(path: &str) -> Result<String, std::io::Error> {
    tokio::fs::read_to_string(Path::new(ULTINA_PAGE_DIR).join(path)).await
}

pub async fn htmx_rsp(path: &str) -> Result<String, std::io::Error> {
    tokio::fs::read_to_string(Path::new(HTMX_RSP_DIR).join(path)).await
}
