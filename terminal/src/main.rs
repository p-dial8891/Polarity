mod tui;
mod polaris;
mod options;

#[tokio::main]
async fn main() {
    tui::app::main().await;
}
