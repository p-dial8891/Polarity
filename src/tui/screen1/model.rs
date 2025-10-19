use crate::tui;
use crate::tui::screen1::{controller::Controller, view::View, 
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
		PlayTrack,
		Draw,
	}		
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use std::collections::HashSet;

#[derive(Clone)]
pub struct Model {
    pub data: polarisHandle,
	pub cmd: ModelCommand
}

pub struct ModelState {
    pub playlist: Rc<HashSet<usize>>,
    pub selection: ListState,
	pub list: Rc<Vec<String>>,
	pub toggle: bool,
	pub tx : Sender<Option<()>>
}

impl<'c> Compute<'c> for Model {
    type State = State;
    type Output = Output;

    async fn compute(
	    self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: [&'c InputPin; 6],
	) -> Output {
		
		let mut state_data = s.unwrap_model();
		
		match self.cmd {
			
			Init => { 
			    state_data.list = Rc::new(polaris::getIterator(self.data.clone())
                .await
                .map(|x| x.0)
                .collect::<Vec<String>>());

                eprintln!("<Model> : intialised.");
			    return Output::View(View {
                    data : self.data,
			        cmd : Draw(
					    state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);
		    },

			PlaybackFinished => {
                eprintln!("<Model> : playback end detected.");
				return Output::View(View { 
				    data : self.data,
				    cmd : PlayTrack(
				        String::from(""),
						state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);
			},

			SelectNext => {
				state_data.selection.select_next();

                eprintln!("<Model> : next track selected.");
			    return Output::View(View {
                    data : self.data,
			        cmd : Draw(
					    state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);
			},

			SelectPrevious => {
				state_data.selection.select_previous();

                eprintln!("<Model> : previous track selected.");
			    return Output::View(View {
                    data : self.data,
			        cmd : Draw(
					    state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);
			},

            AddTrack => {
                Rc::get_mut(&mut state_data.playlist).unwrap()
				    .insert(state_data.selection.selected().unwrap());

                eprintln!("<Model> : New track added to playlist.");
			    return Output::View(View {
                    data : self.data,
			        cmd : Draw(
					    state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);				
            },				

            RemoveTrack => {
                Rc::get_mut(&mut state_data.playlist).unwrap()
				    .remove(&state_data.selection.selected().unwrap());

                eprintln!("<Model> : Track removed from playlist.");
			    return Output::View(View {
                    data : self.data,
			        cmd : Draw(
					    state_data.list.clone(),
						state_data.selection.clone(),
                        state_data.playlist.clone()	) 
					} 
				);				
            },	

            TogglePlay => {
                state_data.toggle = !state_data.toggle;

                eprintln!("<Model> : Play button toggled.");
			    return Output::View(View {
                    data : self.data,
			        cmd : ViewNoop 
					} 
				);
			},

			_ => {	
			
                //eprintln!("<Model> : Noop.");			
			    return Output::View(View {
                    data : self.data,
			        cmd : ViewNoop 
					} 
				);		
			}
		}
    }
}
