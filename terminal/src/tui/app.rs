use crate::tui::{self, Components, Compute, IntoComponent};
use crate::tui::{screen1, screen1::Screen1};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{App_List};
use crate::tui::input::{Input, InputConfig};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, Gpio};
use std::{thread, time::Duration};
use crossterm::{
    event::{poll, read, Event, KeyCode}
};
//use futures::{future::FutureExt, select, StreamExt};

pub const UP_KEY : usize = 0;
pub const DOWN_KEY : usize = 1;
pub const LEFT_KEY : usize = 2;
pub const RIGHT_KEY : usize = 3;
pub const REQ_KEY : usize = 4;

pub async fn main() {
    let gpio = Gpio::new().unwrap();
    let up = InputConfig::init(&gpio, 17, 'j');//gpio.get(17).unwrap().into_input();
    let down = InputConfig::init(&gpio, 22, 'k');//gpio.get(22).unwrap().into_input();
    let left = InputConfig::init(&gpio, 27, 'h');//gpio.get(27).unwrap().into_input();
    let right = InputConfig::init(&gpio, 23, 'l');//gpio.get(23).unwrap().into_input();
    let quit = InputConfig::init(&gpio, 5, '\u{0009}');//gpio.get(5).unwrap().into_input();
    let req = InputConfig::init(&gpio, 6, '\u{000A}');//gpio.get(6).unwrap().into_input();
    let keys = [up, down, left, right, req];
	let mut input	 = Input::init(keys);

    let mut t = ratatui::init();
    t.clear();
	
	let mut a = App_List(Vec::new());

    // Screen definitions - start
    let mut e1 = screen1::Executor { 
		screen_name: a.enumerate("Main"), 
		current_output: None, 
		current_screen: Screen1::new() 
	};

    let mut e2 = shutdown::Executor { 
	    screen_name: a.enumerate("Shutdown"), 
		current_output: None, 
		current_screen: Shutdown::new() 
	};
	// Screen definitions - end
	
	let mut i = a.get_iter();
    let mut i_previous = None;
	let mut i_next = Some(i.next());
	
    loop {
		if poll(Duration::from_millis(5)).unwrap() {
			input.set_event(read().unwrap());
		}
		
		let next = i_next.unwrap().unwrap();
		
        if i_previous != i_next {
			// Screen Initialisations - start
			e1.init(next).await;
			e2.init(next).await;
			// Screen Initialisations - end
		    i_previous = i_next.clone();
		}
		
		// Screen execution - start
		e1.execute(next, &mut t, &mut input).await;
		e2.execute(next, &mut t, &mut input).await;
		// Screen execution - end

		if quit.read(&mut input.ev) == 0.into() {
		    i_previous = i_next.clone();
			i_next = Some(i.next());
		}
			
        thread::sleep(Duration::from_millis(250));
    }
}
