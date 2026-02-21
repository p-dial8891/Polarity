use crate::tui::screen1::{background::model::Model, ModelCommand, ControllerCommand};
use crate::tui::{Compute};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::screen1::{State, OutputBG};

#[derive(Clone)]
pub struct Controller {
	pub cmd: ControllerCommand,
}

impl Compute for Controller {
    type State = State;
    type Output = OutputBG;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Self::Output {
		let state_data = s;
		
		if let Some(t) = &state_data.task {
			if t.is_finished() {
				state_data.task = None;
				return Self::Output::Model(Model { 
			        cmd : ModelCommand::PlaybackFinished	});
			}
		}
		
		match state_data.rx.try_recv() {
			Ok(t_handle) => { state_data.task = t_handle; }
			_ => {}
		}
		// should not matter what happens from here.	
        Self::Output::Model(Model { 
			cmd : ModelCommand::Noop	})
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
			cmd : ControllerCommand::Init,
		}
		
	}
	
}