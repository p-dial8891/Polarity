use crate::tui::home::{background::controller::Controller,
    ViewCommand::{self, Draw, NextTrack},
    ControllerCommand::{self}
};
use crate::tui::{Components, Compute,};
use crate::tui::input::Input;
use crate::tui::home::{State, OutputBG};
use crate::options;
use std::collections::VecDeque;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use service::{PlayerClient};
use std::{time::Duration, time::Instant};
use tarpc::{client, context, tokio_serde::formats::Json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{net::{TcpListener, TcpStream}, task, time::sleep};

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
				let mut mp_address = options::getMicroPolarityAddress();
				// mp_address.extend([":1234"]);
                let mut state_data = s;
				let stream = TcpStream::connect(&mp_address).await.unwrap();
				let _ = state_data.tx.send(Some(task::spawn(playbackTask(stream, name))));
				let _ = state_data.tx_refresh.send(());
                // sendRequestToPlayer(name).await;
            },

            Draw => {
                let mut state_data = s;
                let _ = state_data.tx_refresh.send(());
                eprintln!("<View><Background> : Refresh command sent.");
            },
            
            _ => {}
		}
		
		Self::Output::Controller(Controller { 
		    cmd : ControllerCommand::Noop })
    }
}
