use crate::tui::screen1::{foreground::controllers::{Controller1, Controller2},
    ViewCommand::{self, Init, Draw, PlayTrack},
    ControllerCommand::{self}
};
use crate::tui::{Components, Compute, Render};
use crate::tui::input::Input;
use crate::tui::screen1::{State, Output1, Output2};
use crate::polaris::{self, polarisHandle};
use ratatui::{DefaultTerminal};
use crate::options;

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::AsyncReadExt;
use tokio::{task, net::TcpListener, time::sleep};

#[derive(Clone)]
pub struct View1 {
    pub data: polarisHandle,
	pub cmd: ViewCommand,
}

#[derive(Clone)]
pub struct View2 {
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

impl Compute for View1 {
    type State = State;
    type Output = Output1;

    async fn compute(
        mut self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: &mut Input,
    ) -> Self::Output {
		let mut state_data = s;

		match self.cmd {
			Init => {
			    terminal.clear();
				Self::Output::Controller(Controller1 { 
					cmd : ControllerCommand::Noop,
					data : self.data,
				    redraw : true	})
			},

			Draw => {
				Self::Output::Controller(Controller1 { 
					cmd : ControllerCommand::Noop,
					data : self.data,
				    redraw : true	})
            },

		    PlayTrack(name) => {
				let mut tui_address = options::getTuiAddress();
				tui_address.extend([":9000"]);
				let listener = TcpListener::bind(&tui_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(listenerTask(listener))));
				sendRequestToPlayer(name).await;
				Self::Output::Controller(Controller1 { 
					cmd : ControllerCommand::Noop,
					data : self.data,
				    redraw : false	})
			},
			
            _ => {
				Self::Output::Controller(Controller1 { 
					cmd : ControllerCommand::Noop,
					data : self.data,
				    redraw : false	})
			}			
		}
    }
}


impl Compute for View2 {
    type State = State;
    type Output = Output2;
    
    async fn compute(
        mut self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: &mut Input,
    ) -> Self::Output {
		let mut state_data = s;

		match self.cmd {
			Init => {
				Self::Output::Controller(Controller2 { 
					cmd : ControllerCommand::Noop,
				    redraw : true	})
			},
			
			Draw => {
				Self::Output::Controller(Controller2 { 
					cmd : ControllerCommand::Noop,
				    redraw : true	})
            },
		    PlayTrack(name) => {
				let mut tui_address = options::getTuiAddress();
				tui_address.extend([":9000"]);
				let listener = TcpListener::bind(&tui_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(listenerTask(listener))));
				sendRequestToPlayer(name).await;

				Self::Output::Controller(Controller2 { 
					cmd : ControllerCommand::Noop,
				    redraw : true	})
			},

            _ => {
				Self::Output::Controller(Controller2 { 
					cmd : ControllerCommand::Noop,
				    redraw : false	})
			}			
		}
    }
}

