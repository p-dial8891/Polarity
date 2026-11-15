use crate::tui::search::{models::{Model1, Model2}, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, Render};
use ratatui::{Frame};
use crate::tui::output::Terminal as DefaultTerminal;
use crate::tui::input::Input;
use crate::polaris::{self, polarisHandle};
use crate::tui::search::{State, Output1, Output2};
use crate::tui::app::Keys::{*};

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::options;
use std::collections::VecDeque;

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use std::sync::Arc;
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::AsyncReadExt;
use tokio::{task, net::TcpListener, time::sleep};
use futures::{future::FutureExt, select, StreamExt};

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_model: &Vec<(usize,String)>,
    l_playlist: &VecDeque<usize>,
) {
    let list =
        List::new(list_model.iter().map( |(i, x)| {
                //if l_playlist.iter().position( |x| { x == &i } ).is_some() {
				if l_playlist.contains(&i) {
                    ListItem::new(x.as_str()).yellow()
                } else {
                    ListItem::new(x.as_str()).white()
                }
            },
        ))
        //.highlight_style(SELECTED_STYLE);
		.scroll_padding(2)
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
	
}

/// Render a bottom-to-top list.
pub fn render_bottom(
    frame: &mut Frame, 
	area: Rect, 
	auto_play: bool, 
	l_playlist: &VecDeque<usize>,
	list_state: &mut ListState,
	info_field: &String
) {
	use Constraint::{Length, Fill};

    let autoplay;
    let q_pos;
    autoplay = match auto_play {
        false => { " " },
        true => { "A" }
    };
	let curr_selection = list_state.selected().unwrap_or(0);
	q_pos = match l_playlist.iter().position( |x| { x == &curr_selection } ) {
		Some(i) => i+1,
		None => 0
	};

	let vertical = Layout::vertical([Length(1), Fill(1)]);
	let [top, bottom] = vertical.areas(area);

	let mut final_text_1 = String::new();
	final_text_1.extend([info_field,""]);
	let text_1 = Paragraph::new(final_text_1);
	
	let mut final_text_2 = String::new();
	final_text_2.extend([autoplay, " ", 
		&format!("{:>3}", l_playlist.len()), " ", 
		&format!("{:>3}", q_pos)]);
    let text_2 = Paragraph::new(final_text_2).centered();

	frame.render_widget(text_1, top);
	frame.render_widget(text_2, bottom);
}


async fn sendRequestToPlayer(path: String) {
    //init_tracing("Polarity example.");
    //println!("Polarity example");
	let mut player_address = options::getPlayerAddress();

    let mut transport = tarpc::serde_transport::tcp::connect(
        (player_address, 50051),
        Json::default,
    );
    transport.config_mut().max_frame_length(usize::MAX);
    let client =
        PlayerClient::new(client::Config::default(), transport.await.unwrap())
            .spawn();

    let mut cxt = context::current();
    cxt.deadline = Instant::now()
        .checked_add(Duration::from_secs(60 * 5))
        .unwrap();
    let result = client.play(cxt, path).await.unwrap();
    //println!("{result}");

    sleep(Duration::from_millis(10)).await;
}

async fn listenerTask(listener : TcpListener) {
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut buf = [0; 1];
    socket.read(&mut buf).await.unwrap();
}

impl Render<State> for Controller1 {
    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {
		let mut store_ptr = &mut state.local_store;
		let mut search_store = Arc::get_mut(store_ptr).unwrap().unwrap_search();
        |f,r| { 
			render_list( f, r, &mut search_store.selection, 
		    &search_store.filtered_list, &state.playlist ); 
		}
    }

	fn redraw(&self) -> bool {
		self.redraw
	}
}

impl Render<State> for Controller2 {
    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {
		let mut store_ptr = &mut state.local_store;
		let mut search_store = Arc::get_mut(store_ptr).unwrap().unwrap_search();
        |f,r| { 
			render_bottom( f, r, state.toggle, &state.playlist, 
			&mut search_store.selection, &search_store.ascii_buf_with_cursor ); 
		}
    }

	fn redraw(&self) -> bool {
		self.redraw
	}
}

#[derive(Clone)]
pub struct Controller1 {
	pub cmd: ControllerCommand,
    pub data: polarisHandle,
	pub redraw : bool,
}

#[derive(Clone)]
pub struct Controller2 {
	pub cmd: ControllerCommand,
	pub redraw : bool,
}

impl Compute for Controller1 {
    type State = State;
    type Output = Output1;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Self::Output {

		let mut state_data = s;
		let mut store_ptr = &mut state_data.local_store;
		let mut search_store = Arc::get_mut(store_ptr).unwrap().unwrap_search();

		select!{
			_ = async {
				search_store.edit_len +=	input
					.read_edit(&mut search_store.buffer[
						<i16 as TryInto<usize>>::try_into(search_store.edit_len).unwrap()..])
					.await
					.unwrap();
				search_store.edit_len = if search_store.edit_len < 0 { 
					0 
				} else { 
					search_store.edit_len 
				};
				search_store.edit_len = if search_store.edit_len > 128 { 
					128 
				} else { 
					search_store.edit_len 
				};
			}.fuse() => {
				search_store.ascii_buf = 
					search_store.buffer[..<i16 as TryInto<usize>>::try_into(search_store.edit_len).unwrap()]
					.iter().map(|c| { *c as char })
					.collect::<String>();
				search_store.ascii_buf_with_cursor.clear();
				search_store.ascii_buf_with_cursor.extend([&search_store.ascii_buf, "_"]);
				search_store.cursor = false;	
				if input.read(UP_KEY) == false {
					eprintln!("<Controller> : Up key pressed.");
					return Self::Output::Model(Model1 { data : self.data,
						cmd : ModelCommand::SelectPrevious	});
				}
				if input.read(DOWN_KEY) == false {
					eprintln!("<Controller> : Down key pressed.");
					return Self::Output::Model(Model1 { data : self.data,
						cmd : ModelCommand::SelectNext	});
				}
				if input.read(LEFT_KEY) == false {
					eprintln!("<Controller> : Left key pressed.");
					return Self::Output::Model(Model1 { data : self.data,
						cmd : ModelCommand::RemoveTrack	});
				}
				if input.read(RIGHT_KEY) == false {
					eprintln!("<Controller> : Right key pressed.");
					return Self::Output::Model(Model1 { data : self.data,
						cmd : ModelCommand::AddTrack	});
				}
				return Self::Output::Model(Model1 { data : self.data,
			        cmd : ModelCommand::SearchUpdate});
			},
			_ = async {
				tokio::time::sleep(Duration::from_millis(500)).await;
			}.fuse() => {
				search_store.ascii_buf = 
					search_store.buffer[..<i16 as TryInto<usize>>::try_into(search_store.edit_len).unwrap()]
					.iter().map(|c| { *c as char })
					.collect::<String>();
				search_store.ascii_buf_with_cursor.clear();													
				if search_store.cursor == true {
					search_store.ascii_buf_with_cursor.extend([&search_store.ascii_buf, "_"]);
				} else {
					search_store.ascii_buf_with_cursor.extend([&search_store.ascii_buf, ""]);
				}
				search_store.cursor = !search_store.cursor;
				return Self::Output::Model(Model1 { data : self.data,
			        cmd : ModelCommand::Refresh	});				
			}
		}
		// eprint!("\x1b[2K{}\r", state_data.ascii_buf);

		match self.cmd {
			ControllerCommand::Init => { 
			   state_data.start = false;
			   return Self::Output::Model(Model1 { data : self.data,
			    cmd : ModelCommand::Init	}) },
			_ => {},
		}
	
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Self::Output::Model(Model1 { data : self.data,
			    cmd : ModelCommand::Init	});
		}

		match state_data.rx_refresh.try_recv() {
			Ok(t_handle) => { 
				eprintln!("<Controller> : Refresh command received.");
				return Self::Output::Model(Model1 { data : self.data,
			        cmd : ModelCommand::Refresh	}) },
			Err(e) => { /*eprintln!("{:?}",e)*/}
		}

		// should not matter what happens from here.	
        Self::Output::Model(Model1 { data : self.data,
			cmd : ModelCommand::Noop	})
    }
}

impl Compute for Controller2 {
    type State = State;
    type Output = Output2;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Self::Output {
		let state_data = s;

		match self.cmd {
			ControllerCommand::Init => { 
			   state_data.start = false;
			   return Self::Output::Model(Model2 { 
			    cmd : ModelCommand::Init	}) },
			_ => {},
		}
		
		if input.read(REQ_KEY) == false {
			eprintln!("<Controller> : Request key pressed.");
			return Self::Output::Model(Model2 {
			    cmd : ModelCommand::TogglePlay	});
		}
	
		// should not matter what happens from here.	
        Self::Output::Model(Model2 {
			cmd : ModelCommand::Noop	})
    }
}

impl Controller1 {
	pub async fn new() -> Self {
		Controller1 {
			cmd : ControllerCommand::Init,
		    data : polaris::getBody().await.unwrap(),
			redraw : true,
		}
	}
}

impl Controller2 {
	pub async fn new() -> Self {	
		Controller2 {
			cmd : ControllerCommand::Init,
			redraw : true,
		}
	}
}
