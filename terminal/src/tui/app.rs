use crate::tui::{Components, ExecutorForLayout1, ExecutorForLayout2, ExecutorForBackground};
use crate::tui::{home, home::Home};
use crate::tui::{search};
use crate::tui::{shutdown, shutdown::Shutdown};
use crate::tui::{playback, playback::{Executor}};
use crate::tui::{playlist, playlist::{Executor as PlaylistExecutor}};
use crate::tui::{App_List};
use crate::tui::input::{Input, InputConfig};
use crate::tui::app::Keys::{*};
use crate::tui::menu::{MenuLevel, MenuLevels};
use crossterm::{
	ExecutableCommand,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    event::{poll, read, Event, KeyCode, EventStream},
};
use ratatui::{
	Terminal,
	backend::CrosstermBackend
};
use std::{thread, time::Duration, rc::Rc, io::Write};
use tokio::task::{spawn};
use tokio::fs::File;
use futures::{future::FutureExt, select, StreamExt};
use crate::options;

pub enum Keys {
	UP_KEY = 0,
	DOWN_KEY = 1,
	LEFT_KEY = 2,
	RIGHT_KEY = 3,
	REQ_KEY = 4,
	TAB_KEY = 5,
	FIND_KEY = 6
}

async fn getDisplayFd() -> impl Write {
	let mut tty_async = File::open(options::getDisplay().as_str()).await.unwrap();
	let mut tty = tty_async.try_into_std().unwrap();
	tty
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

    let mut t_console = ratatui::init();
    t_console.clear();

	let mut backend = CrosstermBackend::new(getDisplayFd().await);
	let mut t_display = Terminal::new(backend);
	
	let mut a = App_List(Vec::new());

    // Configuration - start

	a.register("Main");
	a.register("Search");
	a.register("Playlist");
	a.register("Playback");
	let shutdown_screen = a.register("Shutdown");

	let mut home = Home::new();

	let mut e0 = home::ExecutorBG { 
		controllers: None
	};

    let mut e1 = home::Executor { 
		controllers: (None,None) 
	};

    let mut e2 = search::Executor { 
		controllers: (None,None) 
	};

	let mut e3_display = playlist::Executor { 
		controllers: (None,None), 
	};

	let mut e3_console = playlist::Executor { 
		controllers: (None,None), 
	};

	let mut e4 = playback::Executor { 
		controllers: (None,None), 
	};
	
    let mut e5 = shutdown::Executor { 
	    screen_names: vec![shutdown_screen], 
		current_output: None, 
		current_screen: Shutdown::new() 
	};

	const menu_1 : MenuLevel = MenuLevel::Level1("Main");
	const menu_2 : MenuLevel = MenuLevel::Level2("Main", KeyCode::Char('f'), KeyCode::Esc);
	const menu_3 : MenuLevel = MenuLevel::Level1("Playlist");
	const menu_4 : MenuLevel = MenuLevel::Level1("Playback");
	const menu_5 : MenuLevel = MenuLevel::Level1("Shutdown");

	let menus = &[menu_1,menu_2,menu_3,menu_4,menu_5];
	let mut menu_iter = MenuLevels {
		c: menus.iter().cycle(),
		size: 5,
		input_set: &[KeyCode::Tab, KeyCode::Esc, KeyCode::Char('f')]
	};
    // Configuration - end
    
	enable_raw_mode().unwrap();
	getDisplayFd().await.execute(EnterAlternateScreen);

	let mut m = menu_1;

	e0.init().await;

	loop {
		match m {
			menu_1 =>  {
				(e0,m,menu_iter,e1,input,t_console,home) = spawn(async move {
					let mut reader = EventStream::new();
					e1.init().await;
					loop {
						e0.execute(&mut home.v, &mut t_console, &mut input).await;
						m = m.visit(&mut menu_iter, &mut input);
						if m == menu_1 {
							let mut event = reader.next().fuse();
							select! {
								ev = event => { 
									match ev {
										Some(Ok(e)) => { input.set_event(e); },
										_ => {}
									}
								},
								_ = async {
									tokio::time::sleep(Duration::from_millis(5)).await;
								}.fuse() => {}
							}
							e1.execute(&mut home.v, &mut t_console, &mut input).await;
						}
						else {
							break;
						}
						tokio::time::sleep(Duration::from_millis(100)).await;
					}
					(e0,m,menu_iter,e1,input,t_console,home)
				}).await.unwrap();
			},

			menu_2 =>  {
				(e0,m,menu_iter,e2,input,t_console,home) = spawn(async move {
					e2.init().await;
					t_console.clear();
					loop {
						e0.execute(&mut home.v, &mut t_console, &mut input).await;
						m = m.visit(&mut menu_iter, &mut input);
 						if m == menu_2 {
							e2.execute(&mut home.v, &mut t_console, &mut input).await;
						}
						else {
							break;
						}
						// tokio::time::sleep(Duration::from_millis(100)).await;
					}
					(e0,m,menu_iter,e2,input,t_console,home)
				}).await.unwrap();
			},

			menu_3 =>  {
				(e0,m,menu_iter,e3_console,input,t_console,home) = spawn(async move {
					let mut reader = EventStream::new();
					e3_console.init().await;
					loop {
						e0.execute(&mut home.v, &mut t_console, &mut input).await;
						m = m.visit(&mut menu_iter, &mut input);
						if m == menu_3 {
							let mut event = reader.next().fuse();
							select! {
								ev = event => { 
									match ev {
										Some(Ok(e)) => { input.set_event(e); },
										_ => {}
									}
								},
								_ = async {
									tokio::time::sleep(Duration::from_millis(5)).await;
								}.fuse() => {}
							}
							e3_console.execute(&mut home.v, &mut t_console, &mut input).await;
						}
						else {
							break;
						}
						tokio::time::sleep(Duration::from_millis(100)).await;
					}
					(e0,m,menu_iter,e3_console,input,t_console,home)
				}).await.unwrap();
			},

			menu_4 =>  {
				(e0,m,menu_iter,e4,input,t_console,home) = spawn(async move {
					let mut reader = EventStream::new();
					e4.init().await;
					loop {
						e0.execute(&mut home.v, &mut t_console, &mut input).await;
						m = m.visit(&mut menu_iter, &mut input);
						if m == menu_4 {
							let mut event = reader.next().fuse();
							select! {
								ev = event => { 
									match ev {
										Some(Ok(e)) => { input.set_event(e); },
										_ => {}
									}
								},
								_ = async {
									tokio::time::sleep(Duration::from_millis(5)).await;
								}.fuse() => {}
							}
							e4.execute(&mut home.v, &mut t_console, &mut input).await;
						}
						else {
							break;
						}
						tokio::time::sleep(Duration::from_millis(100)).await;
					}
					(e0,m,menu_iter,e4,input,t_console,home)
				}).await.unwrap();
			},

			menu_5 =>  {
				(e0,m,menu_iter,e5,input,t_console,home) = spawn(async move {
					let mut reader = EventStream::new();
					e5.init(&String::from("Shutdown")).await;
					loop {
						e0.execute(&mut home.v, &mut t_console, &mut input).await;
						m = m.visit(&mut menu_iter, &mut input);
						if m == menu_5 {
							let mut event = reader.next().fuse();
							select! {
								ev = event => { 
									match ev {
										Some(Ok(e)) => { input.set_event(e); },
										_ => {}
									}
								},
								_ = async {
									tokio::time::sleep(Duration::from_millis(5)).await;
								}.fuse() => {}
							}
							e5.execute(&String::from("Shutdown"), &mut t_console, &mut input).await;
						}
						else {
							break;
						}
						tokio::time::sleep(Duration::from_millis(100)).await;
					}
					(e0,m,menu_iter,e5,input,t_console,home)
				}).await.unwrap();
			},

			_ => { },
		}
	}

	getDisplayFd().await.execute(LeaveAlternateScreen).unwrap();
	disable_raw_mode().unwrap();
}
