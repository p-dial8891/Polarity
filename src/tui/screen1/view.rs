use crate::tui;
use crate::tui::screen1::{controller::Controller, model::Model,
    ViewCommand::{self, Init, Draw, PlayTrack}};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use rppal::gpio::{self, InputPin};
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashSet;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use service::{PlayerClient, init_tracing};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{net::SocketAddr, time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::AsyncReadExt;
use tokio::{net::TcpListener, task, time::sleep};

#[derive(Clone)]
pub struct View {
    pub data: polarisHandle,
	pub cmd: ViewCommand,
}

pub struct ViewState {
    pub s: u16,
    pub b: u64,
	pub tx : Sender<Option<task::JoinHandle<()>>>
}


/// Render the UI with various lists.
fn render(
    frame: &mut Frame,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    toggle_play: bool,
    l_playlist: &HashSet<usize>,
) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(8), Length(2)]);
    let [top, bottom] = vertical.areas(frame.area());

    render_list(frame, top, list_state, list_model, l_playlist);
    render_bottom(frame, bottom, toggle_play);
}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    l_playlist: &HashSet<usize>,
) {
    let list =
        List::new(list_model.into_iter().map(|x| x.as_str()).enumerate().map(
            |(i, x)| {
                if l_playlist.contains(&i) {
                    ListItem::new(x).yellow()
                } else {
                    ListItem::new(x).white()
                }
            },
        ))
        //.highlight_style(SELECTED_STYLE);
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_bottom(frame: &mut Frame, area: Rect, auto_play: bool) {
    let final_text;
    match auto_play {
        false => final_text = String::from("\n             "),
        true => {
            let mut temp_text = String::from("\n             ");
            temp_text.extend(["A"]);
            final_text = temp_text;
        }
    }
    let text = Paragraph::new(final_text);
    frame.render_widget(text, area);
}


async fn sendRequestToPlayer(path: String) {
    //init_tracing("Polarity example.");
    //println!("Polarity example");

    let mut transport = tarpc::serde_transport::tcp::connect(
        ("raspberrypi.local", 50051),
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

async fn listenerTask() {
    let listener = TcpListener::bind("raspberrypi.local:9000").await.unwrap();
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut buf = [0; 1];
    socket.read(&mut buf).await.unwrap();
}

impl<'c> Compute<'c> for View {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: [&'c InputPin; 6],
    ) -> Output {
		
		match self.cmd {
			Init(data, mut list_state, playlist, toggle_symbol) => {
			    terminal.clear();	
    			terminal.draw(|frame| {
				    render(frame, &mut list_state, &data, toggle_symbol, &playlist) }).unwrap();
			},
			
		    PlayTrack(name, data, mut list_state, playlist, toggle_symbol) => {
                let state_data = s.unwrap_view();
				let _ = state_data.tx.send(Some(task::spawn(listenerTask())));
				sendRequestToPlayer(name).await;
                terminal.draw(|frame| {
				    render(frame, &mut list_state, &data, toggle_symbol, &playlist) }).unwrap();
            },
			
			Draw(data, mut list_state, playlist, toggle_symbol) => {
    			terminal.draw(|frame| {
				    render(frame, &mut list_state, &data, toggle_symbol, &playlist) }).unwrap();
            },

            _ => {}			
		}
		
		Output::Controller(Controller { data : self.data })
    }
}
