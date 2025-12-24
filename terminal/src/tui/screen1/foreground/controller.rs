use crate::tui;
use crate::tui::screen1::{foreground::model::Model, foreground::view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::polaris::{self, polarisHandle};
use crate::tui::screen1::{State, Output};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::tui::app::Keys::{self, *};
use std::process::Command;
use tokio::task;
use ratatui::widgets::{ListState};


#[derive(Clone)]
pub struct Controller {
	pub cmd: ControllerCommand,
    pub data: polarisHandle,
}

impl<'c> Compute<'c> for Controller {
    type State = State;
    type Output = Output;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Output {
		let state_data = s;

		match self.cmd {
			ControllerCommand::Init => { 
			   state_data.start = false;
			   return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::Init	}) },
			_ => {},
		}
	
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::Init	});
		}

		match state_data.rx_refresh.try_recv() {
			Ok(t_handle) => { 
				eprintln!("<Controller> : Refresh command received.");
				return Output::Model(Model { data : self.data,
			        cmd : ModelCommand::Refresh	}) }
			_ => {}
		}

		if input.read(UP_KEY) == false {
			eprintln!("<Controller> : Up key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::SelectPrevious	});
		}
		if input.read(DOWN_KEY) == false {
			eprintln!("<Controller> : Down key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::SelectNext	});
		}
		if input.read(LEFT_KEY) == false {
			eprintln!("<Controller> : Left key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::RemoveTrack	});
		}
		if input.read(RIGHT_KEY) == false {
			eprintln!("<Controller> : Right key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::AddTrack	});
		}
		if input.read(REQ_KEY) == false {
			eprintln!("<Controller> : Request key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::TogglePlay	});
		}
		// should not matter what happens from here.	
        Output::Model(Model { data : self.data,
			cmd : ModelCommand::Noop	})
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
			cmd : ControllerCommand::Init,
		    data : polaris::getBody().await.unwrap(),
		}
		
	}
	
}