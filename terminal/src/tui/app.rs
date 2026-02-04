use crate::tui::{Components, ExecutorForLayout1, ExecutorForLayout2, ExecutorForBackground};
use crate::tui::{screen1, screen1::Screen1};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{playback, playback::{Playback,Executor}};
use crate::tui::{App_List};
use crate::tui::input::{Input, InputConfig};
use std::{thread, time::Duration};
use std::rc::Rc;
use crossterm::{
    event::{poll, read}
};
use crate::tui::app::Keys::{*};
use crossterm::{
    event::{KeyCode}
};
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
    let up = InputConfig::init(17, KeyCode::Up);
    let down = InputConfig::init(22, KeyCode::Down);
    let left = InputConfig::init(27, KeyCode::Left);
    let right = InputConfig::init(23, KeyCode::Right);
    let quit = InputConfig::init(5, KeyCode::Tab);
    let req = InputConfig::init(6, KeyCode::Enter);
    let keys = [up, down, left, right, req, quit];
	let mut input = Input::init(keys);

    let mut t = ratatui::init();
    t.clear();
	
	let mut a = App_List(Vec::new());

    // Screen definitions - start

	a.register("Main");
	a.register("Playback");
	let shutdown_screen = a.register("Shutdown");

	let mut screen1 = Screen1::new();
	let mut playback = Playback::new();

	let mut e0 = screen1::ExecutorBG { 
		controllers: None
	};

    let mut e1 = screen1::Executor { 
		controllers: (None,None) 
	};

	let mut e2 = playback::Executor { 
		controllers: (None,None), 
	};
	
    let mut e3 = shutdown::Executor { 
	    screen_names: vec![shutdown_screen], 
		current_output: None, 
		current_screen: Shutdown::new() 
	};
	// Screen definitions - end
	
	let mut i = a.get_iter();
    let mut i_previous = None;
	let mut i_next = Some(i.next());
	
	e0.init().await;
	
    loop {
        if poll(Duration::from_millis(5)).unwrap() {
		    input.set_event(read().unwrap());
		}
		
		e0.execute(&mut screen1.v, &mut t, &mut input).await;
		
		let next = i_next.unwrap().unwrap();
		
        if i_previous != i_next {
			// Screen Initialisations - start
			match &next[..] {
			    "Main"     => { e1.init().await; },
				"Playback" => { e2.init().await; },
				"Shutdown" => { e3.init(next).await; },
				_          => {},
			}
			// Screen Initialisations - end
		    i_previous = i_next;
		}
		
		// Screen execution - start
		match &next[..] {
		    "Main"     => { e1.execute(&mut screen1.v, &mut t, &mut input).await;  },
			"Playback" => {	e2.execute(&mut playback.v, &mut t, &mut input).await; },
			"Shutdown" => { e3.execute(next, &mut t, &mut input).await; },
			_          => {},
		}
		// Screen execution - end

		if input.read(TAB_KEY) == false {
		    i_previous = i_next;
			i_next = Some(i.next());
		}
			
        thread::sleep(Duration::from_millis(100));
    }
}
