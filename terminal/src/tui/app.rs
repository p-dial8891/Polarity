use crate::tui::{self, Components, Compute, IntoComponent};
use crate::tui::{screen1, screen1::Screen1};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{App_List};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, Gpio, InputPin};
use std::{thread, time::Duration};

pub const UP_KEY : usize = 0;
pub const DOWN_KEY : usize = 1;
pub const LEFT_KEY : usize = 2;
pub const RIGHT_KEY : usize = 3;
pub const REQ_KEY : usize = 4;

pub async fn main() {
    let gpio = Gpio::new().unwrap();
    let up = gpio.get(17).unwrap().into_input();
    let down = gpio.get(22).unwrap().into_input();
    let left = gpio.get(27).unwrap().into_input();
    let right = gpio.get(23).unwrap().into_input();
    let quit = gpio.get(5).unwrap().into_input();
    let req = gpio.get(6).unwrap().into_input();
    let g = [&up, &down, &left, &right, &req];

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
		let next = i_next.unwrap().unwrap();
		
        if i_previous != i_next {
			// Screen Initialisations - start
			e1.init(next).await;
			e2.init(next).await;
			// Screen Initialisations - end
		    i_previous = i_next.clone();
		}
		
		// Screen execution - start
		e1.execute(next, &mut t, g.clone()).await;
		e2.execute(next, &mut t, g.clone()).await;
		// Screen execution - end

		if quit.read() == 0.into() {
		    i_previous = i_next.clone();
			i_next = Some(i.next());
		}
			
        thread::sleep(Duration::from_millis(250));
    }
}
