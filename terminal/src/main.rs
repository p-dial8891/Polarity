mod tui;
mod polaris;

#[tokio::main]
async fn main() {
    tui::app::main().await;
}
