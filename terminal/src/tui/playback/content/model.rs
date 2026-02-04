use crate::tui;
use crate::tui::playback::{content::controller::Controller, content::view::View, 
    ModelCommand::{
		self, 
		Noop,
		Init,
		SelectPrevious,
		SelectNext,
		Req
	}, 
	ViewCommand::{
		self,
		Noop as ViewNoop,
		Init as ViewInit,
		Skip,
		Pause
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
			
            SelectPrevious => {
				let i = match s.selection.selected() {
					Some(i) => {
						if i == 0 {
							1
						} else {
							i - 1
						}
					}
					None => 0,
				};
				s.selection.select(Some(i));
                eprintln!("<Model> : previous selected.");
			    return Self::Output::View(View { cmd : ViewCommand::Draw } );
			},

			SelectNext => {
				let i = match s.selection.selected() {
					Some(i) => {
						if i >= 1 {
							0
						} else {
							i + 1
						}
					}
					None => 0,
				};
				s.selection.select(Some(i));
                eprintln!("<Model> : next selected.");
			    return Self::Output::View(View { cmd : ViewCommand::Draw } );
			},

			Req => {			
			    match s.selection.selected() {
					Some(0) => {
						eprintln!("<Model> : skip requested.");
				        return Output::View(View { cmd : Skip } )
					},

				    Some(1) => { 
						eprintln!("<Model> : pause requested.");
				        return Output::View(View { cmd : Pause } ) 
					},

					_ => { return Output::View(View { cmd : ViewNoop } ) }
			    }
			},

			_ =>  { return Output::View(View { cmd : ViewNoop } ) }
			
		}
    }
}
