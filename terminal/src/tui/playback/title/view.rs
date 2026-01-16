use crate::tui;
use crate::tui::playback::{title::controller::Controller, title::model::Model,
    ViewCommand::{self, Init, Skip},
	ControllerCommand::{self, Noop}
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Render};
use crate::tui::playback::{State, Output1 as Output};
use crate::options;
use ratatui::DefaultTerminal;
use crate::tui::input::Input;

use tarpc::{client, context, tokio_serde::formats::Json};
use service::{PlayerClient};
use std::{net::SocketAddr, time::Duration, time::Instant};
use tokio::{net::TcpListener, task, time::sleep};

#[derive(Clone)]
pub struct View {
    pub cmd : ViewCommand,
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
					redraw : true	})
			},

			_ => {
				Output::Controller(Controller { cmd : ControllerCommand::Noop,
					redraw : false	})
			}
		}

    }
}
