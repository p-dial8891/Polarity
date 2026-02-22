use crate::tui::screen1::{background::view::View, 
    ModelCommand::{
		self, 
		PlaybackFinished,
	}, 
	ViewCommand::{
		Noop as ViewNoop,
		NextTrack,
		Draw,
	}		
};
use crate::tui::{Compute};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::screen1::{State, OutputBG};
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Model {
	pub cmd: ModelCommand
}

async fn getNextTrack(list: &Vec<(String,String)>, s: &VecDeque<usize>) -> String {
    //let mut list_polaris = polaris::getIterator(h).await;
    let index = s.get(0).unwrap();
    list.iter().nth(*index).unwrap().1.clone()
}

impl Compute for Model {
    type State = State;
    type Output = OutputBG;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Self::Output {
		
		let mut state_data = s;
		
		match self.cmd {

			PlaybackFinished => {
				state_data.playlist.pop_front();
				
				if state_data.toggle && !state_data.playlist.is_empty() {
    				let next = getNextTrack(&state_data.polaris_data, &state_data.playlist).await;
					eprintln!("<Model> : Next track selected {}",next);
    				return Self::Output::View(View { 
		    		    cmd : NextTrack(
						    next
					    )
					} 
					);
				} else 
				if state_data.toggle {
					state_data.toggle = false;
					return Self::Output::View(View {
						cmd : Draw	} 
					);
				}
				
				return Self::Output::View(View {
					cmd : ViewNoop 
					} 
				);
			},

			_ => {	
			
                //eprintln!("<Model> : Noop.");			
			    return Self::Output::View(View {
			        cmd : ViewNoop 
					} 
				);		
			}
		}
    }
}
