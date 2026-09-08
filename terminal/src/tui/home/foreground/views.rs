use crate::tui::home::{foreground::controllers::{Controller1, Controller2},
    ViewCommand::{self, Init, Draw, PlayTrack},
    ControllerCommand::{self}
};
use crate::tui::{Components, Compute, Render};
use crate::tui::input::Input;
use crate::tui::home::{State, Output1, Output2};
use crate::polaris::{self, polarisHandle};
use ratatui::{DefaultTerminal};
use crate::options;

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{task, net::{TcpListener, TcpStream}, time::sleep};

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


async fn playbackTask(mut stream : TcpStream, name : String) {
    // let (mut socket, _) = listener.accept().await.unwrap();
    let mut buf = [0; 1];
    if name.as_str().starts_with("AwsMusic/Music/") {
        let mut extracted_name = String::from(&name["AwsMusic/Music/".len()..]);
		extracted_name.extend(["\r"]);
        eprintln!("Sending \"{}\" to micropolarity", extracted_name);
        stream.write_all(extracted_name.as_bytes()).await;
    } else {
        eprintln!("Track name did not start with the right prefix location.");
        return;
    }
    loop {
        match stream.read(&mut buf).await {
            Ok(0) => { break; },
            _     => { 
                eprintln!("Received unexpected end of stream data.");
                break;
            },
        };
    }
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
				let mut mp_address = options::getMicroPolarityAddress();
				// mp_address.extend([":1234"]);
				let stream = TcpStream::connect(&mp_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(playbackTask(stream, name))));
				let _ = state_data.tx_refresh.send(());
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
				let mut mp_address = options::getMicroPolarityAddress();
				// mp_address.extend([":1234"]);
				let stream = TcpStream::connect(&mp_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(playbackTask(stream, name))));
				let _ = state_data.tx_refresh.send(());

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

