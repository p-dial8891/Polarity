use crate::tui;
use crate::tui::shutdown::{controller::Controller, view::View, 
    ModelCommand::{
		self, 
		Noop,
		Init,
	}, 
	ViewCommand::{
		self,
		Noop as ViewNoop,
		Init as ViewInit,
	}
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::shutdown::{State, Output};
use crate::tui::input::Input;

#[derive(Clone)]
pub struct Model {
    pub cmd : ModelCommand
}

pub struct ModelState {
    pub _a : ()
}

impl Compute for Model {
    type State = State;
    type Output = Output;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Output {
        match self.cmd {
			
			Init => { return Output::View(View { cmd : ViewInit } ) },
			
			_ =>  { return Output::View(View { cmd : ViewNoop } ) }
			
		}
    }
}
