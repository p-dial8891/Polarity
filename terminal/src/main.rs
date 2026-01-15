#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
//#![allow(warnings)]

mod tui;
mod polaris;
mod options;

#[tokio::main]
async fn main() {
    tui::app::main().await;
}
