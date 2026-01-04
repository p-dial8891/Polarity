use crate::tui;
use crate::tui::playback::{title::model::Model, title::view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::playback::{State, Output1 as Output};
use crate::tui::app::Keys::{self, *};
use std::process::Command;
use ratatui::widgets::{ListState};

#[derive(Clone)]
pub struct Controller {
    pub cmd : ControllerCommand,
	pub redraw : bool
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
		
		match self.cmd {
			
			ControllerCommand::Init => {
				Output::Model(Model { cmd : ModelCommand::Init	})
			}
			
			ControllerCommand::Noop => {
				Output::Model(Model { cmd : ModelCommand::Noop	})				
			}
		}
	}
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init,
			redraw: true,
		}
		
	}
	
}