use crate::tui::screen1::{foreground::controller::Controller,
    ViewCommand::{self},
    ControllerCommand::{self}
};
use crate::tui::{Components, Compute, Render};
use crate::tui::input::Input;
use crate::tui::screen1::{State, Output};
use crate::polaris::{polarisHandle};
use crate::options;
use std::collections::VecDeque;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::AsyncReadExt;
use tokio::{net::TcpListener, time::sleep};

#[derive(Clone)]
pub struct View {
    pub data: polarisHandle,
	pub cmd: ViewCommand,
}

/// Render the UI with various lists.
fn render_test (
    s : & mut State,
	terminal: &mut DefaultTerminal,
) {
    use Constraint::{Fill, Length};

    let vertical = Layout::vertical([Fill(1), Length(2)]);
    let [top, bottom] = vertical.areas(terminal.get_frame().area());

    {
	    let a = View::renderer(s);
	    a(&mut terminal.get_frame(), top);
	}
    {
	    let b = View::renderer(s);
	    b(&mut terminal.get_frame(), bottom);
	}

}

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

impl Render<State> for View {

    fn renderer<'a>(state : &'a mut State) -> 
	    Box<dyn FnOnce(&mut Frame<'a>, Rect) -> () +'_> {

        Box::new( |f,r| { render_list( f, r, &mut state.selection, 
		                        &state.list, &state.playlist ); } )
		
    }
}

impl<'c> Compute<'c> for View {
    type State = State;
    type Output = Output;

    async fn compute(
        mut self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: &mut Input,
    ) -> Output {
		let mut state_data = s;
/*
		match self.cmd {
			Init(data, playlist, toggle_symbol) => {
			    terminal.clear();	
    			terminal.draw(|frame| {
				    render(frame, &mut state_data.selection, &data, toggle_symbol, &playlist) }).unwrap();
			},
			
			Draw(data, playlist, toggle_symbol) => {
    			terminal.draw(|frame| {
				    render(frame, &mut state_data.selection, &data, toggle_symbol, &playlist) }).unwrap();
            },
		    PlayTrack(name, data, playlist, toggle_symbol) => {
				let mut tui_address = options::getTuiAddress();
				tui_address.extend([":9000"]);
				let listener = TcpListener::bind(&tui_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(listenerTask(listener))));
				sendRequestToPlayer(name).await;
                terminal.draw(|frame| {
				    render(frame, &mut state_data.selection, &data, toggle_symbol, &playlist) }).unwrap();
			},
			
            _ => {}			
		}
*/	
        
		render_test(state_data, terminal);
		render_test(state_data, terminal);
		
		Output::Controller(Controller { 
		    cmd : ControllerCommand::Noop,
			data : self.data	})
    }
}


