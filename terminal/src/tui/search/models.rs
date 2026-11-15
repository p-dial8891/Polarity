use crate::tui::search::{views::{View1,View2}, 
    ModelCommand::{
		self, 
		Init,
		SelectNext,
		SelectPrevious,
		AddTrack,
		RemoveTrack,
		TogglePlay,
		Refresh,
		SearchUpdate
	},
	ViewCommand::{
		Noop as ViewNoop,
		Init as ViewInit,
		PlayTrack,
		Draw,
	}
};
use crate::tui::{Components, Compute};
use crate::tui::output::Terminal as DefaultTerminal;
use crate::tui::input::Input;
use crate::tui::search::{State, Output1, Output2};
use crate::polaris::{self, polarisHandle};
use std::sync::Arc;
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

pub struct SearchStore {
	pub filtered_list : Vec<(usize, String)>,
	pub edit_len: i16,
	pub buffer: [u8;128],
	pub cursor: bool,
	pub ascii_buf: String,
	pub ascii_buf_with_cursor: String,
	pub selection: ListState
}

pub type Data = SearchStore;

async fn getNextTrack(list: &Vec<(String,String)>, s: &VecDeque<usize>) -> String {
    //let mut list_polaris = polaris::getIterator(h).await;
    let index = s.get(0).unwrap();
    list.iter().nth(*index).unwrap().1.clone()
}

impl Compute for Model1 {
    type State = State;
    type Output = Output1;

    async fn compute(
	    mut self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: &mut Input,
	) -> Self::Output {
		
		let mut state_data = s;
		let mut store_ptr = &mut state_data.local_store;
		let mut search_store = Arc::get_mut(store_ptr).unwrap().unwrap_search();

		match self.cmd {
			
			Init => { 
			    state_data.polaris_data = polaris::getIterator(self.data.clone())
					.await
					.collect::<Vec<(String,String)>>();
				
			    state_data.list = polaris::getIterator(self.data.clone())
					.await
					.map(|x| x.0)
					.collect::<Vec<String>>();

                eprintln!("<Model> : intialised.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : ViewInit
					} 
				);
		    },

			SelectNext => {
				let i = match search_store.selection.selected() {
					Some(i) => {
						if i >= state_data.list.len() - 1 {
							0
						} else {
							i + 1
						}
					}
					None => 0,
				};
				search_store.selection.select(Some(i));
                eprintln!("<Model> : next track selected.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);
			},

			SelectPrevious => {
				let i = match search_store.selection.selected() {
					Some(i) => {
						if i == 0 {
							state_data.list.len() - 1
						} else {
							i - 1
						}
					}
					None => 0,
				};
				search_store.selection.select(Some(i));
                eprintln!("<Model> : previous track selected.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);
			},

            AddTrack => {
				state_data.playlist.push_back(
					search_store.filtered_list.get(
						search_store.selection.selected().unwrap()
					).unwrap().0 // index
				);

                eprintln!("<Model> : New track added to playlist.");
			    return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw
					} 
				);				
            },				

            RemoveTrack => {
				if let Some(i) = state_data.playlist.iter().position(
				    |x| { *x == (search_store.filtered_list.get(
							search_store.selection.selected().unwrap()
						).unwrap().0) } 
				) {
				    state_data.playlist.remove(i);
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

			SearchUpdate => {
				eprintln!("<Model> : Filtering list.");
				search_store.filtered_list = state_data.list
					.iter().enumerate()
					.filter(|(i,x)| { x.to_lowercase().contains(&search_store.ascii_buf.to_lowercase()) })
					.map(|(i,x)| {(i,x.clone())})
					.collect::<Vec<(usize,String)>>();

				return Self::Output::View(View1 {
                        data : self.data,
			            cmd : Draw 
					} 
				);
			}

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

impl Compute for Model2 {
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
    				let next = getNextTrack(&state_data.polaris_data, &state_data.playlist).await;
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

impl SearchStore {
	pub fn new() -> Self {
		SearchStore {
			filtered_list : Vec::new().into_iter().enumerate().collect::<Vec<(usize,String)>>(),
			edit_len : 0,
			buffer : [0u8;128],
			cursor : false,
			ascii_buf : String::new(),
			ascii_buf_with_cursor : String::new(),
			selection : ListState::default().with_selected(Some(0))
		}
	}
}