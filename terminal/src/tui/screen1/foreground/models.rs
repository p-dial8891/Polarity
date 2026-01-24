use crate::tui::screen1::{foreground::views::{View1,View2}, 
    ModelCommand::{
		self, 
		Init,
		SelectNext,
		SelectPrevious,
		AddTrack,
		RemoveTrack,
		TogglePlay,
		Refresh 
	}, 
	ViewCommand::{
		Noop as ViewNoop,
		Init as ViewInit,
		PlayTrack,
		Draw,
	}		
};
use crate::tui::{Components, Compute,};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::screen1::{State, Output1, Output2};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver};
use ratatui::widgets::{ListState};
use std::collections::VecDeque;
use tokio::task;

#[derive(Clone)]
pub struct Model1 {
    pub data: polarisHandle,
	pub cmd: ModelCommand
}


#[derive(Clone)]
pub struct Model2 {
	pub cmd: ModelCommand
}

pub struct ComponentState {
    pub start: bool,
	pub task: Option<task::JoinHandle<()>>,
	pub rx: Receiver<Option<task::JoinHandle<()>>>,
	pub rx_refresh: Receiver<()>,
    pub playlist: Rc<VecDeque<usize>>,
	pub polaris_data : Rc<Vec<(String,String)>>,
	pub list: Rc<Vec<String>>,
	pub toggle: bool,
	pub tx : Sender<Option<task::JoinHandle<()>>>,
	pub tx_refresh: Sender<()>,
	pub selection: ListState,
}

async fn getNextTrack(list: Rc<Vec<(String,String)>>, s: &VecDeque<usize>) -> String {
    //let mut list_polaris = polaris::getIterator(h).await;
    let index = s.get(0).unwrap();
    list.iter().nth(*index).unwrap().1.clone()
}

impl<'c> Compute<'c> for Model1 {
    type State = State;
    type Output = Output1;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Self::Output {
		
		let mut state_data = s;
		
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
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : ViewInit
					} 
				);
		    },

			SelectNext => {
				let i = match state_data.selection.selected() {
					Some(i) => {
						if i >= state_data.list.len() - 1 {
							0
						} else {
							i + 1
						}
					}
					None => 0,
				};
				state_data.selection.select(Some(i));
                eprintln!("<Model> : next track selected.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);
			},

			SelectPrevious => {
				let i = match state_data.selection.selected() {
					Some(i) => {
						if i == 0 {
							state_data.list.len() - 1
						} else {
							i - 1
						}
					}
					None => 0,
				};
				state_data.selection.select(Some(i));
                eprintln!("<Model> : previous track selected.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);
			},

            AddTrack => {
				let mut p: &mut VecDeque<usize> = 
				    Rc::get_mut(&mut state_data.playlist).unwrap();
				p.push_back(state_data.selection.selected().unwrap());

                eprintln!("<Model> : New track added to playlist.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);				
            },				

            RemoveTrack => {
				let mut p: &mut VecDeque<usize> = 
				    Rc::get_mut(&mut state_data.playlist).unwrap();
				if let Some(i) = p.iter().position(
				    |x| { *x == state_data.selection.selected().unwrap() } 
				) {
				    p.remove(i);
				}

                eprintln!("<Model> : Track removed from playlist.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);				
            },	

			Refresh => {
				eprintln!("<Model> : Refreshing view.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw 
					} 
				);
			},
			_ => {	
                //eprintln!("<Model> : Noop.");			
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : ViewNoop 
					} 
				);		
			}
		}
    }
}

impl<'c> Compute<'c> for Model2 {
    type State = State;
    type Output = Output2;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Self::Output {
		
		let mut state_data = s;
		
		match self.cmd {
			
            TogglePlay => {
                state_data.toggle = !state_data.toggle;

				if state_data.toggle && !state_data.playlist.is_empty() {
    				let next = getNextTrack(state_data.polaris_data.clone(), &state_data.playlist).await;
					eprintln!("<Model> : Next track selected {}",next);
    				return Self::Output::View(View2 { 
		    		        cmd : PlayTrack( next ) 
					    } 
					);
				} else 
				if state_data.toggle {
					state_data.toggle = false;
					return Self::Output::View(View2 { 
					    	cmd : Draw
						} 
					);
				}

				return Self::Output::View(View2 {
					    cmd : ViewNoop 
					} 
				);
			},

			_ => {	
			
                //eprintln!("<Model> : Noop.");			
			    return Self::Output::View(View2 {
			        cmd : ViewNoop 
					} 
				);		
			}
		}
    }
}
