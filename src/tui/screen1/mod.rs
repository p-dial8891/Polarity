use crate::tui;
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::polaris;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use std::collections::HashSet;

mod controller;
mod model;
mod view;

use crate::tui::screen1::{
	controller::{
		Controller, ControllerState
	}, 
	model::{
		Model, ModelState
	}, 
	view::{
		View, ViewState
	}
};

pub type State = tui::ComponentData<ModelState, ViewState, ControllerState>;
pub type Output = tui::ComponentData<Model, View, Controller>;

pub struct Screen1 {
    pub v: Vec<State>,
}

impl<'c> Components<'c> for Screen1 {
    type Item = Screen1;
    type Output = Output;

    fn new() -> Screen1 {
		let (tx, rx) = channel();
        Screen1 {
            v: Vec::from([
                State::Controller(ControllerState { 
				    start: true, 
					task: None,
					rx: rx }),
                State::Model(ModelState	{ 
				    playlist: Rc::new(HashSet::new()), 
				    selection : ListState::default().with_selected(    Some(0)),
					list: Rc::new(Vec::new()), 
					toggle: false,
				    tx: tx.clone() }),
                State::View(ViewState { s: 0, b: 0, tx: tx.clone() }),
            ])
        }
    }

    async fn run(
        &mut self,
        o: Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Output {
        o.unwrap_controller()
            .compute(&mut self.v[0], terminal, gpio_pins).await
            .unwrap_model()
            .compute(&mut self.v[1], terminal, gpio_pins).await
            .unwrap_view()
            .compute(&mut self.v[2], terminal, gpio_pins).await
    }

}

impl Screen1 {
    pub async fn start(&mut self) -> Output {
        Output::Controller ( 
		    Controller::new().await
		)
    }
		
}

impl IntoComponent<Model, View, Controller> for Output {
    fn unwrap_controller(self) -> Controller {
        match self {
            Output::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model {
        match self {
            Output::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View {
        match self {
            Output::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}


impl IntoComp<ModelState, ViewState, ControllerState> for State {
    fn unwrap_controller(&mut self) -> &mut ControllerState {
        match self {
            State::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(&mut self) -> &mut ModelState {
        match self {
            State::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(&mut self) -> &mut ViewState {
        match self {
            State::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

#[derive(Clone)]
pub enum ModelCommand {
	
	Noop,
	Init,
	PlaybackFinished,
	SelectNext,
	SelectPrevious,
	AddTrack,
	RemoveTrack,
	TogglePlay,
	
}

#[derive(Clone)]
pub enum ViewCommand {
	
	Noop,
    PlayTrack(String, Rc<Vec<String>>, ListState, Rc<HashSet<usize>>),
	Draw(Rc<Vec<String>>, ListState, Rc<HashSet<usize>>),
}