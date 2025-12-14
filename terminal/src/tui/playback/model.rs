use crate::tui;
use crate::tui::playback::{controller::Controller, view::View, 
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
use crate::tui::playback::{State, Output};
use crate::tui::input::Input;
use ratatui::widgets::{ListState};

#[derive(Clone)]
pub struct Model {
    pub cmd : ModelCommand,
	pub selection : ListState
}

pub struct ModelState {
    pub _a : ()
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
			
			Init => { return Output::View(View { cmd : ViewInit,
                        selection : self.selection } ) },
			
			Req => {				
			    match self.selection {
					_ => { return Output::View(View { cmd : Skip,
				        selection : self.selection } ) }
			    }
			},
			
			_ =>  { return Output::View(View { cmd : ViewNoop,
				        selection : self.selection } ) }
			
		}
    }
}
