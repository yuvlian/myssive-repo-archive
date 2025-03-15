use crate::models::Report;

pub async fn on_report() -> Report {
    Report::from("GET LOG")
}
