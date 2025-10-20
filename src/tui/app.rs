use crate::tui::{self, Components, Compute, IntoComponent, screen1, screen1::Screen1};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, Gpio, InputPin};
use std::{thread, time::Duration};

pub const UP_KEY : usize = 0;
pub const DOWN_KEY : usize = 1;
pub const LEFT_KEY : usize = 2;
pub const RIGHT_KEY : usize = 3;
pub const QUIT_KEY : usize = 4;
pub const REQ_KEY : usize = 5;

pub async fn main() {
    let gpio = Gpio::new().unwrap();
    let up = gpio.get(17).unwrap().into_input();
    let down = gpio.get(22).unwrap().into_input();
    let left = gpio.get(27).unwrap().into_input();
    let right = gpio.get(23).unwrap().into_input();
    let quit = gpio.get(5).unwrap().into_input();
    let req = gpio.get(6).unwrap().into_input();
    let g = [&up, &down, &left, &right, &quit, &req];
    let mut t = ratatui::init();

    let mut s1 = Screen1::new();
    let i1 = s1.start().await;

    loop {
        let i1 = s1.run(i1.clone(), &mut t, g.clone()).await;
        thread::sleep(Duration::from_millis(250));
    }
}
