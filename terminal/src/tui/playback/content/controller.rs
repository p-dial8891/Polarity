use crate::tui;
use crate::tui::playback::{content::model::Model, content::view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::playback::{State, Output2 as Output};
use crate::tui::app::Keys::{self, *};
use std::process::Command;
use ratatui::widgets::{ListState};

#[derive(Clone)]
pub struct Controller {
    pub cmd : ControllerCommand,
	pub redraw : bool,
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
		
		let mut state_data = s;
		
		match self.cmd {
			ControllerCommand::Init => {
				state_data.start = false;
				eprintln!("<Controller> : Initialised.");
				return Output::Model(Model { cmd : ModelCommand::Init });
			},
			_ => {}
		}
		
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { cmd : ModelCommand::Init });
		}

 		if input.read(REQ_KEY) == false {
			eprintln!("<Controller> : Req key pressed.");
			return Output::Model(Model { cmd : ModelCommand::Req });
		}
		
		Output::Model(Model { cmd : ModelCommand::Noop })
			
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init,
			redraw: true
		}
		
	}
	
}