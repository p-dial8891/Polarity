mod tui;
mod polaris;
mod options;


#[allow(non_camel_case_types)]
#[tokio::main]
async fn main() {
    tui::app::main().await;
}
