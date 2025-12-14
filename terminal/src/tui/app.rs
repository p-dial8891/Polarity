use crate::tui::{self, Components, Compute, IntoComponent};
use crate::tui::{screen1, screen1::Screen1};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{playback, playback::Playback};
use crate::tui::{App_List};
use crate::tui::input::{Input, InputConfig};
use ratatui::DefaultTerminal;
use std::{thread, time::Duration};
use crossterm::{
    event::{poll, read, Event, KeyCode}
};
use crate::tui::app::Keys::{*};
//use futures::{future::FutureExt, select, StreamExt};

pub enum Keys {
	UP_KEY = 0,
	DOWN_KEY = 1,
	LEFT_KEY = 2,
	RIGHT_KEY = 3,
	REQ_KEY = 4,
	TAB_KEY = 5
}

pub async fn main() {
    let up = InputConfig::init(17, 'j');
    let down = InputConfig::init(22, 'k');
    let left = InputConfig::init(27, 'h');
    let right = InputConfig::init(23, 'l');
    let quit = InputConfig::init(5, '\u{0009}');
    let req = InputConfig::init(6, '\u{000A}');
    let keys = [up, down, left, right, req, quit];
	let mut input = Input::init(keys);

    let mut t = ratatui::init();
    t.clear();
	
	let mut a = App_List(Vec::new());

    let main_bg = String::from("Main_in_background");
	
    // Screen definitions - start
    let mut e0 = screen1::Executor { 
		screen_names: vec![a.register("Main"), main_bg.clone()], 
		current_output: None, 
		current_screen: Screen1::new() 
	};
	
	let mut e1 = e0.with_background();
	
    let mut e2 = playback::Executor { 
	    screen_names: vec![a.register("Playback")], 
		current_output: None, 
		current_screen: Playback::new() 
	};
	
    let mut e3 = shutdown::Executor { 
	    screen_names: vec![a.register("Shutdown")], 
		current_output: None, 
		current_screen: Shutdown::new() 
	};
	// Screen definitions - end
	
	let mut i = a.get_iter();
    let mut i_previous = None;
	let mut i_next = Some(i.next());
	
	e1.init(&main_bg).await;
	
    loop {
		if poll(Duration::from_millis(5)).unwrap() {
			input.set_event(read().unwrap());
		}
		
		e1.execute(&main_bg, &mut t, &mut input).await;
		
		let next = i_next.unwrap().unwrap();
		
        if i_previous != i_next {
			// Screen Initialisations - start
			e1.foreground_executor.init(next).await;
			e2.init(next).await;
			e3.init(next).await;
			// Screen Initialisations - end
		    i_previous = i_next.clone();
		}
		
		// Screen execution - start
		e1.foreground_executor.execute(next, &mut t, &mut input).await;
		e2.execute(next, &mut t, &mut input).await;
		e3.execute(next, &mut t, &mut input).await;
		// Screen execution - end

		if input.read(TAB_KEY) == false {
		    i_previous = i_next.clone();
			i_next = Some(i.next());
		}
			
        thread::sleep(Duration::from_millis(250));
    }
}
