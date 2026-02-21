use crate::tui;
use crate::tui::{Components, Compute, IntoComponent, IntoComp, ExecutorForLayout1, ExecutorForBackground};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use std::rc::Rc;
use std::sync::mpsc::{channel};
use ratatui::widgets::{ListState};
use std::collections::VecDeque;

pub mod background;
pub mod foreground;

use crate::tui::screen1::foreground::{
	controllers::{
		Controller1, Controller2
	}, 
	models::{
		Model1, Model2, ComponentState
	}, 
	views::{
		View1, View2
	}
};

use crate::tui::screen1::background::{
	controller::{
		Controller as ControllerBG, 
	}, 
	model::{
		Model as ModelBG, 
	}, 
	view::{
		View as ViewBG, 
	}
};

pub type State = ComponentState;
pub type Output1 = tui::ComponentData<Model1, View1, Controller1>;
pub type Output2 = tui::ComponentData<Model2, View2, Controller2>;
pub type OutputBG = tui::ComponentData<ModelBG, ViewBG, ControllerBG>;

pub struct Screen1 {
    pub v: State,
}

impl Screen1 {
	
    pub fn new() -> Self {
		let (tx, rx) = channel();
        let (tx_refresh, rx_refresh) = channel();
        Screen1 {
            v: ComponentState { 
				start: true, 
				task: None,
				rx: rx,
				rx_refresh: rx_refresh,
				playlist: Rc::new(VecDeque::new()), 
				polaris_data: Rc::new(Vec::new()),
				list: Rc::new(Vec::new()), 
				toggle: false,
				tx: tx.clone(),
				tx_refresh: tx_refresh.clone(),
				selection: ListState::default().with_selected(Some(0)),
                playback: foreground::models::PlaybackState {
                    start: true, 
                    selection: ListState::default().with_selected(Some(0))
                }
            }
        }
    }
}

impl IntoComponent<Model1, View1, Controller1> for Output1 {
    fn unwrap_controller(self) -> Controller1 {
        match self {
            Output1::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model1 {
        match self {
            Output1::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View1 {
        match self {
            Output1::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

impl IntoComponent<Model2, View2, Controller2> for Output2 {
    fn unwrap_controller(self) -> Controller2 {
        match self {
            Output2::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model2 {
        match self {
            Output2::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View2 {
        match self {
            Output2::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

impl IntoComponent<ModelBG, ViewBG, ControllerBG> for OutputBG {
    fn unwrap_controller(self) -> ControllerBG {
        match self {
            OutputBG::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> ModelBG {
        match self {
            OutputBG::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> ViewBG {
        match self {
            OutputBG::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

#[derive(Clone)]
pub enum ControllerCommand {
	
	Noop,
	Init,
	
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
    Refresh
	
}

#[derive(Clone)]
pub enum ViewCommand {
	
	Noop,
	Init,
    NextTrack(String),
    PlayTrack(String),
	Draw,
}

pub struct Executor {
	pub controllers: (Option<Output1>, Option<Output2>),
}

pub struct ExecutorBG {
	pub controllers: Option<OutputBG>,
}

impl ExecutorForLayout1 <
    State, 
    Output1, 
    Output2, 
    Model1, 
    Model2, 
    View1, 
    View2, 
    Controller1, 
    Controller2 
> 
  for Executor
{

    fn get_controllers(&self) -> (Output1, Output2) {
        (
            self.controllers.0.clone().unwrap(),
            self.controllers.1.clone().unwrap()
        )
    }

    fn set_controllers(&mut self, controllers : (Output1, Output2)) {
        self.controllers.0 = Some(controllers.0);
        self.controllers.1 = Some(controllers.1);
    }

    async fn init(&mut self) {
        self.set_controllers((
            Output1::Controller( Controller1::new().await ),
            Output2::Controller( Controller2::new().await )
        ));    
    }

}

impl ExecutorForBackground <
	State, 
	OutputBG, 
	ModelBG, 
	ViewBG, 
	ControllerBG
> 
  for ExecutorBG
{

	fn get_controller(&self) -> OutputBG {
		self.controllers.clone().unwrap()
	}

	fn set_controller(&mut self, controller : OutputBG) {
		self.controllers = Some(controller);
	}

	async fn init(&mut self) {
		self.set_controller(
			OutputBG::Controller( ControllerBG::new().await )
		);    
	}

}
