use crate::tui;
use crate::tui::screen1::{foreground::controller::Controller, foreground::view::View, 
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
		Draw,
	}		
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use std::collections::VecDeque;
use tokio::task;

#[derive(Clone)]
pub struct Model {
    pub data: polarisHandle,
    pub selection: ListState,
	pub cmd: ModelCommand
}

pub struct ModelState {
    pub playlist: Rc<VecDeque<usize>>,
	pub polaris_data : Rc<Vec<(String,String)>>,
	pub list: Rc<Vec<String>>,
	pub toggle: bool,
	pub tx : Sender<Option<task::JoinHandle<()>>>
}

async fn getNextTrack(list: Rc<Vec<(String,String)>>, s: &VecDeque<usize>) -> String {
    //let mut list_polaris = polaris::getIterator(h).await;
    let index = s.get(0).unwrap();
    list.iter().nth(*index).unwrap().1.clone()
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
		
		let mut state_data = s.unwrap_model();
		
		match self.cmd {
			
			Init => { 
			    state_data.polaris_data = Rc::new(polaris::getIterator(self.data.clone())
                .await
                .collect::<Vec<(String,String)>>());
				
			    state_data.list = Rc::new(polaris::getIterator(self.data.clone())
                .await
                .map(|x| x.0)
                .collect::<Vec<String>>());

                eprintln!("<Model> : intialised.");
			    return Output::View(View {
                    data : self.data,
					selection : self.selection,
			        cmd : ViewInit(
						state_data.list.clone(),
						state_data.playlist.clone(),
						state_data.toggle )
					} 
				);
		    },

			SelectNext => {
				let i = match self.selection.selected() {
					Some(i) => {
						if i >= state_data.list.len() - 1 {
							0
						} else {
							i + 1
						}
					}
					None => 0,
				};
				self.selection.select(Some(i));
                eprintln!("<Model> : next track selected.");
			    return Output::View(View {
                    data : self.data,
					selection : self.selection,
			        cmd : Draw(
					    state_data.list.clone(),
                        state_data.playlist.clone(),
                        state_data.toggle ) 
					} 
				);
			},

			SelectPrevious => {
				let i = match self.selection.selected() {
					Some(i) => {
						if i == 0 {
							state_data.list.len() - 1
						} else {
							i - 1
						}
					}
					None => 0,
				};
				self.selection.select(Some(i));
                eprintln!("<Model> : previous track selected.");
			    return Output::View(View {
                    data : self.data,
                    selection : self.selection,
			        cmd : Draw(
					    state_data.list.clone(),
                        state_data.playlist.clone(),
                        state_data.toggle ) 
					} 
				);
			},

            AddTrack => {
				let mut p = Rc::get_mut(&mut state_data.playlist).unwrap();
				p.push_back(self.selection.selected().unwrap());

                eprintln!("<Model> : New track added to playlist.");
			    return Output::View(View {
                    data : self.data,
					selection : self.selection,
			        cmd : Draw(
					    state_data.list.clone(),
                        state_data.playlist.clone(),
                        state_data.toggle ) 
					} 
				);				
            },				

            RemoveTrack => {
				let mut p = Rc::get_mut(&mut state_data.playlist).unwrap();
				let i = p.iter().position(
				    |x| { *x == self.selection.selected().unwrap() } 
				).unwrap();
				p.remove(i);

                eprintln!("<Model> : Track removed from playlist.");
			    return Output::View(View {
                    data : self.data,
					selection : self.selection,
			        cmd : Draw(
					    state_data.list.clone(),
                        state_data.playlist.clone(),
                        state_data.toggle ) 
					} 
				);				
            },	

            TogglePlay => {
                state_data.toggle = !state_data.toggle;

				if state_data.toggle && !state_data.playlist.is_empty() {
    				let next = getNextTrack(state_data.polaris_data.clone(), &state_data.playlist).await;
					eprintln!("<Model> : Next track selected {}",next);
    				return Output::View(View { 
	    			    data : self.data,
					    selection : self.selection,
		    		    cmd : PlayTrack(
			    	        next,
				    		state_data.list.clone(),
							state_data.playlist.clone(),
							state_data.toggle ) 
					    } 
					);
				} else 
				if { state_data.toggle } {
					state_data.toggle = false;
					return Output::View(View {
						data : self.data,
						selection : self.selection,
						cmd : Draw(
							state_data.list.clone(),
							state_data.playlist.clone(),
							state_data.toggle ) 
						} 
					);
				}

				return Output::View(View {
					data : self.data,
					selection : self.selection,
					cmd : ViewNoop 
					} 
				);
			},

			_ => {	
			
                //eprintln!("<Model> : Noop.");			
			    return Output::View(View {
                    data : self.data,
					selection : self.selection,
			        cmd : ViewNoop 
					} 
				);		
			}
		}
    }
}
