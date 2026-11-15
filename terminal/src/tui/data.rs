use std::sync::mpsc::{Sender, Receiver};
use ratatui::widgets::{ListState};
use std::collections::VecDeque;
use std::vec::Vec;
use std::sync::mpsc::{channel};
use std::sync::Arc;
use std::mem;
use tokio::task;
use crate::tui::SwapStore;

use crate::tui::playlist::models::Data as PlaylistData;

pub enum ComponentStore {
    Empty,
    Playlist(PlaylistData)
}

pub struct PlaybackState {
    pub start : bool,
	pub selection : ListState
}

pub struct Data {
    pub v: State,
}

pub struct State {
    pub start: bool,
	pub task: Option<task::JoinHandle<()>>,
	pub rx: Receiver<Option<task::JoinHandle<()>>>,
	pub rx_refresh: Receiver<()>,	
    pub playlist: VecDeque<usize>,
	pub polaris_data : Vec<(String,String)>,
	pub list: Vec<String>,
	pub toggle: bool,
	pub tx : Sender<Option<task::JoinHandle<()>>>,
	pub tx_refresh: Sender<()>,
	pub display_refresh: bool,
	pub selection: ListState,
    pub local_store: Arc<ComponentStore>,
	pub playback: PlaybackState,
	pub filtered_list : Vec<(usize, String)>,
	pub edit_len: i16,
	pub buffer: [u8;128],
	pub cursor: bool,
	pub ascii_buf: String,
	pub ascii_buf_with_cursor: String
}

impl SwapStore for State {
	fn swap_selection(&mut self, other : &mut Arc<ComponentStore>) {
		mem::swap(&mut self.local_store, other);
	}
}

impl Data {
	
    pub fn new() -> Self {
		let (tx, rx) = channel();
        let (tx_refresh, rx_refresh) = channel();
        Data {
            v: State { 
				start: true, 
				task: None,
				rx: rx,
				rx_refresh: rx_refresh,
				playlist: VecDeque::new(), 
				polaris_data: Vec::new(),
				list: Vec::new(), 
				toggle: false,
				tx: tx.clone(),
				tx_refresh: tx_refresh.clone(),
                display_refresh: false,
				selection: ListState::default().with_selected(Some(0)),
                local_store: Arc::new(ComponentStore::new()),
                playback: PlaybackState {
                    start: true, 
                    selection: ListState::default().with_selected(Some(0))
                },
                filtered_list : Vec::new().into_iter().enumerate().collect::<Vec<(usize,String)>>(),
                edit_len : 0,
		        buffer : [0u8;128],
		        cursor : false,
                ascii_buf : String::new(),
                ascii_buf_with_cursor : String::new()
            }
        }
    }
}

impl ComponentStore {
    pub fn unwrap_playlist(&mut self) -> &mut PlaylistData {
        match self {
            ComponentStore::Playlist(x) => { x },
            _ => { panic!("Component state Wrong type.") }
        }
    }

    pub fn new() -> Self {
        ComponentStore::Empty
    }

    pub fn new_playlist() -> Self {
        ComponentStore::Playlist(PlaylistData::new())
    }
}