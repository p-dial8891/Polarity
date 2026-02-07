use crate::tui;
use crate::tui::playback::{title::controller::Controller, title::view::View, 
    ModelCommand::{
		self, 
		Noop,
		Init,
		Req
	}, 
	ViewCommand::{
		self,
		Noop as ViewNoop,
		Init as ViewInit,
		Skip
	}		
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::playback::{State, Output1 as Output};
use crate::tui::input::Input;
use ratatui::widgets::{ListState};

#[derive(Clone)]
pub struct Model {
    pub cmd : ModelCommand,

}

impl<'c> Compute<'c> for Model {
    type State = State;
    type Output = Output;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Output {
		
		match self.cmd {
			
			ModelCommand::Init => { Output::View(View { cmd : ViewInit } ) },
				
			_ => { Output::View(View { cmd : ViewNoop } ) },
		}
    }
}
