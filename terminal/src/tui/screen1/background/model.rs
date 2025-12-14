use crate::tui;
use crate::tui::screen1::{background::controller::Controller, background::view::View, 
    ModelCommand::{
		self, 
		Noop,
		Init,
		PlaybackFinished,
		SelectNext,
		SelectPrevious,
		AddTrack,
		RemoveTrack,
		TogglePlay 
	}, 
	ViewCommand::{
		self, 
		Noop as ViewNoop,
		Init as ViewInit,
		PlayTrack,
		NextTrack,
		Draw,
	}		
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::screen1::{State, OutputBG};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use std::collections::VecDeque;
use tokio::task;

#[derive(Clone)]
pub struct Model {
	pub cmd: ModelCommand
}

async fn getNextTrack(list: Rc<Vec<(String,String)>>, s: &VecDeque<usize>) -> String {
    //let mut list_polaris = polaris::getIterator(h).await;
    let index = s.get(0).unwrap();
    list.iter().nth(*index).unwrap().1.clone()
}

impl<'c> Compute<'c> for Model {
    type State = State;
    type Output = OutputBG;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Self::Output {
		
		let mut state_data = s.unwrap_model();
		
		match self.cmd {

			PlaybackFinished => {
			    Rc::get_mut(&mut state_data.playlist).unwrap().pop_front();
				
				if state_data.toggle && !state_data.playlist.is_empty() {
    				let next = getNextTrack(state_data.polaris_data.clone(), &state_data.playlist).await;
					eprintln!("<Model> : Next track selected {}",next);
    				return Self::Output::View(View { 
		    		    cmd : NextTrack(next) 
					    } 
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
