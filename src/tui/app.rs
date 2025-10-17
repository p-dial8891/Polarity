use crate::tui::{self, Components, Compute, IntoComponent, screen1, screen1::Screen1};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, Gpio, InputPin};
use std::{thread, time::Duration};

pub fn main() {
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

    let i1 = s1.start();

    let mut s2 = Screen1::new();

    let i2 = s2.start();

    loop {
        let i1 = s1.run(i1.clone(), &mut t, g.clone());
        let i2 = s2.run(i2.clone(), &mut t, g.clone());
        thread::sleep(Duration::from_millis(250));
    }
}
