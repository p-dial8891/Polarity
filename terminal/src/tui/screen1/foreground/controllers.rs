use crate::tui::screen1::{foreground::models::{Model1, Model2}, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, Render};
use ratatui::{DefaultTerminal, Frame};
use crate::tui::input::Input;
use crate::polaris::{self, polarisHandle};
use crate::tui::screen1::{State, Output1, Output2};
use crate::tui::app::Keys::{*};

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};

use crate::options;
use std::collections::VecDeque;

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::AsyncReadExt;
use tokio::{task, net::TcpListener, time::sleep};

/// Render the UI with various lists.
fn render(
    frame: &mut Frame,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    toggle_play: bool,
    l_playlist: &VecDeque<usize>,
) {
    use Constraint::{Fill, Length};

    let vertical = Layout::vertical([Fill(1), Length(2)]);
    let [top, bottom] = vertical.areas(frame.area());

    render_list(frame, top, list_state, list_model, l_playlist);
    render_bottom(frame, bottom, toggle_play, l_playlist, list_state);
}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    l_playlist: &VecDeque<usize>,
) {
    let list =
        List::new(list_model.into_iter().map(|x| x.as_str()).enumerate().map(
            |(i, x)| {
                //if l_playlist.iter().position( |x| { x == &i } ).is_some() {
				if l_playlist.contains(&i) {
                    ListItem::new(x).yellow()
                } else {
                    ListItem::new(x).white()
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
) {
    let autoplay;
    let q_pos;
    autoplay = match auto_play {
        false => { " " },
        true => { "A" }
    };
	let curr_selection = list_state.selected().unwrap();
	q_pos = match l_playlist.iter().position( |x| { x == &curr_selection } ) {
		Some(i) => i+1,
		None => 0
	};	
	let mut final_text = String::from("\n           ");
	final_text.extend([
		autoplay, " ", 
		&format!("{:>3}", l_playlist.len()), " ", 
		&format!("{:>3}", q_pos)]);
    let text = Paragraph::new(final_text);
    frame.render_widget(text, area);
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

        |f,r| { render_list( f, r, &mut state.selection, 
		                        &state.list, &state.playlist ); }
		
    }

	fn redraw(&self) -> bool {
		self.redraw
	}
}

impl Render<State> for Controller2 {

    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {

        |f,r| { render_bottom( f, r, state.toggle, 
			&state.playlist, &mut state.selection ); }
		
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

		let state_data = s;

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
			        cmd : ModelCommand::Refresh	}) }
			_ => {}
		}

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
