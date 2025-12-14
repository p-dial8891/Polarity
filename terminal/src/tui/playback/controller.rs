use crate::tui;
use crate::tui::playback::{model::Model, view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::playback::{State, Output};
use crate::tui::app::Keys::{self, *};
use std::process::Command;
use ratatui::widgets::{ListState};

#[derive(Clone)]
pub struct Controller {
    pub cmd : ControllerCommand,
	pub selection : ListState
}

pub struct ControllerState {
    pub start : bool
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
		
		let state_data = s.unwrap_controller();
		
		match self.cmd {
			ControllerCommand::Init => {
				state_data.start = false;
				eprintln!("<Controller> : Initialised.");
				return Output::Model(Model { cmd : ModelCommand::Init,
                    selection : self.selection		
				});
			},
			_ => {}
		}
		
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { cmd : ModelCommand::Init,
                selection : self.selection		
			});
		}

 		if input.read(REQ_KEY) == false {
			eprintln!("<Controller> : Req key pressed.");
			return Output::Model(Model { cmd : ModelCommand::Req,
                selection : self.selection		
			});
		}
		
		Output::Model(Model { cmd : ModelCommand::Noop,
            selection : self.selection		
		})
			
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init,
			selection : ListState::default().with_selected(Some(0))
		}
		
	}
	
}