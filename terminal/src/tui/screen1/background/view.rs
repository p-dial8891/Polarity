use crate::tui::screen1::{background::controller::Controller,
    ViewCommand::{self, Draw, NextTrack},
    ControllerCommand::{self}
};
use crate::tui::{Components, Compute,};
use crate::tui::input::Input;
use crate::tui::screen1::{State, OutputBG};
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
use tokio::{net::TcpListener, task, time::sleep};

#[derive(Clone)]
pub struct View {
	pub cmd: ViewCommand,
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

impl Compute for View {
    type State = State;
    type Output = OutputBG;

    async fn compute(
        mut self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: &mut Input,
    ) -> Self::Output {
		
		match self.cmd {
			NextTrack(name) => {
				let mut tui_address = options::getTuiAddress();
				tui_address.extend([":9000"]);
                let mut state_data = s;
				let listener = TcpListener::bind(&tui_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(listenerTask(listener))));
				let _ = state_data.tx_refresh.send(());
                sendRequestToPlayer(name).await;
            },

            Draw => {
                let mut state_data = s;
                let _ = state_data.tx_refresh.send(());
            },
            
            _ => {}
		}
		
		Self::Output::Controller(Controller { 
		    cmd : ControllerCommand::Noop })
    }
}
