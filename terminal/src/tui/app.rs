use crate::tui::{Components, ExecutorForLayout1, ExecutorForLayout2, ExecutorForBackground};
use crate::tui::{screen1, screen1::Screen1};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{playback, playback::{Executor}};
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
use crate::tui::menu::{MenuLevel, MenuLevels};
//use futures::{future::FutureExt, select, StreamExt};

pub enum Keys {
	UP_KEY = 0,
	DOWN_KEY = 1,
	LEFT_KEY = 2,
	RIGHT_KEY = 3,
	REQ_KEY = 4,
	TAB_KEY = 5,
	FIND_KEY = 6
}

pub async fn main() {
    let up = InputConfig::init(17, KeyCode::Up);
    let down = InputConfig::init(22, KeyCode::Down);
    let left = InputConfig::init(27, KeyCode::Left);
    let right = InputConfig::init(23, KeyCode::Right);
    let quit = InputConfig::init(5, KeyCode::Tab);
    let req = InputConfig::init(6, KeyCode::Enter);
	let find = InputConfig::init(0, KeyCode::Char('f'));
    let keys = [up, down, left, right, req, quit, find];
	let mut input = Input::init(keys);

    let mut t = ratatui::init();
    t.clear();
	
	let mut a = App_List(Vec::new());

    // Configuration - start

	a.register("Main");
	a.register("Playback");
	let shutdown_screen = a.register("Shutdown");

	let mut screen1 = Screen1::new();

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

	const menu_1 : MenuLevel = MenuLevel::Level1("Main");
	const menu_2 : MenuLevel = MenuLevel::Level2(
		"Main",	KeyCode::Char('p'),	KeyCode::Esc
	);
	const menu_3 : MenuLevel = MenuLevel::Level1("Shutdown");
	let menus = &[menu_1,menu_2,menu_3];
	let mut menu_iter = MenuLevels {
		c: menus.iter().cycle(),
		size: 3,
		input_set: &[KeyCode::Tab, KeyCode::Esc, KeyCode::Char('p')]
	};
    // Configuration - end

	let mut m = menu_1;
	let mut m_prev = None;

	e0.init().await;

    loop {
        if poll(Duration::from_millis(5)).unwrap() {
		    input.set_event(read().unwrap());
		}
		
		e0.execute(&mut screen1.v, &mut t, &mut input).await;
		
		m = m.visit(&mut menu_iter, &mut input);	

        if m_prev.is_none() || m_prev.clone().unwrap() != m {
			eprintln!("Menu is {:?}", m);
			// Screen Initialisations - start
			match m {
			    menu_1    	=> { e1.init().await; },
				menu_2		=> { e2.init().await; },
				menu_3		=> { e3.init(&String::from("Shutdown")).await; },
				_          	=> {},
			}
			// Screen Initialisations - end
		    m_prev = Some(m.clone());
		}
		
		// Screen execution - start
		match m {
			menu_1  => { e1.execute(&mut screen1.v, &mut t, &mut input).await; },
			menu_2  => { e2.execute(&mut screen1.v, &mut t, &mut input).await; },
			menu_3  => { e3.execute(&String::from("Shutdown"), &mut t, &mut input).await; },
			_       => {},

		}
		// Screen execution - end

		thread::sleep(Duration::from_millis(100));
    }
}
