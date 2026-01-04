use crate::tui;
use crate::tui::playback::{content::controller::Controller, content::model::Model,
    ViewCommand::{self, Init, Skip},
	ControllerCommand::{self, Noop}
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Render};
use crate::tui::playback::{State, Output2 as Output};
use crate::options;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use crate::tui::input::Input;

use tarpc::{client, context, tokio_serde::formats::Json};
use service::{PlayerClient};
use std::{net::SocketAddr, time::Duration, time::Instant};
use tokio::{net::TcpListener, task, time::sleep};

#[derive(Clone)]
pub struct View {
    pub cmd : ViewCommand,
}

async fn sendRequestToPlayer() {
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
    let result = client.skip(cxt).await.unwrap();
    //println!("{result}");

    sleep(Duration::from_millis(10)).await;
}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
) {
    let list =
        List::new(["Skip"])
        //.highlight_style(SELECTED_STYLE);
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

impl Render<State> for View {

    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {

        move |f,r| { render_list( f, r, &mut state.selection ); }
		
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
		
		match self.cmd {
			
			Init => {
				Output::Controller(Controller { cmd : ControllerCommand::Noop,
                    redraw: true  } )
            },
			
			Skip => {
				sendRequestToPlayer().await;
                Output::Controller(Controller { cmd : ControllerCommand::Noop,
                    redraw: false  } )
			},
			
			_ => {
                Output::Controller(Controller { cmd : ControllerCommand::Noop,
                    redraw: false  } )
            }
		}
    }
}
