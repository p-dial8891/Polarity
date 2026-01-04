use crate::tui;
use crate::tui::playback::{content::controller::Controller, content::view::View, 
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
use crate::tui::playback::{State, Output2 as Output};
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
			
			Init => { return Output::View(View { cmd : ViewInit } ) },
			
			Req => {				
			    match s.selection {
					_ => { return Output::View(View { cmd : Skip } ) }
			    }
			},

			_ =>  { return Output::View(View { cmd : ViewNoop } ) }
			
		}
    }
}
