use crate::tui;
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Execute, ExecuteBG};
use ratatui::DefaultTerminal;
use crate::tui::input::Input;
use crate::polaris;
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use std::collections::VecDeque;

mod background;
mod foreground;

use crate::tui::screen1::foreground::{
	controller::{
		Controller
	}, 
	model::{
		Model, ComponentState
	}, 
	view::{
		View
	}
};

pub type State = ComponentState;
pub type Output = tui::ComponentData<Model, View, Controller>;
pub type Executor<'c> = tui::Execute<'c, Screen1>;

pub struct Screen1 {
    pub v: State,
}

impl<'c> Components<'c> for Screen1 {
    type Item = Screen1;
    type Output = Output;

    fn new() -> Screen1 {
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
				selection: ListState::default().with_selected(Some(0))
            }
        }
    }

    async fn run(
        &mut self,
        o: Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) -> Output {
        o.unwrap_controller()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_model()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_view()
            .compute(&mut self.v, terminal, gpio_pins).await
    }

    async fn start(&mut self) -> Output {
        Output::Controller ( 
		    Controller::new().await
		)
    }
}

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

pub type OutputBG = tui::ComponentData<ModelBG, ViewBG, ControllerBG>;

impl Screen1 {
	
    async fn run_as_background(
        &mut self,
        o: OutputBG,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) -> OutputBG {
        o.unwrap_controller()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_model()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_view()
            .compute(&mut self.v, terminal, gpio_pins).await
    }

    async fn start_as_background(&mut self) -> OutputBG {
        OutputBG::Controller ( 
		    ControllerBG::new().await
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
	Init(Rc<Vec<String>>, Rc<VecDeque<usize>>, bool),
    NextTrack(String),
    PlayTrack(String, Rc<Vec<String>>, Rc<VecDeque<usize>>, bool),
	Draw(Rc<Vec<String>>, Rc<VecDeque<usize>>, bool),
}

impl<'c> Execute<'c,Screen1> {
	pub async fn init(&mut self, handle: &String) {
		if self.screen_names.iter().position(|x| { x == handle }).is_some() {
		    self.current_output = Some(self.current_screen.start().await);
		}
	}

	pub async fn execute(
	    &mut self, 
		handle: &String,
		terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input
	) {
		if self.screen_names.iter().position(|x| { x == handle }).is_some() {
		    self.current_output = Some(
			    self.current_screen.run(self.current_output.clone().unwrap(), 
				terminal, gpio_pins).await
			);
		}
	}
	
	pub fn with_background(&'c mut self) -> ExecuteBG<'c, Screen1, OutputBG> {
		ExecuteBG {
			foreground_executor : self,
			current_output : None
		}
	}
	
}

impl<'c> ExecuteBG<'c, Screen1, OutputBG> {

	pub async fn init(&mut self, handle: &String) {
		if self.foreground_executor.screen_names.iter().position(
		        |x| { x == handle }).is_some() {
		    self.current_output = Some(self.foreground_executor
				.current_screen.start_as_background().await);
		}
	}

	pub async fn execute(
	    &mut self, 
		handle: &String,
		terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input
	) {
		if self.foreground_executor.screen_names.iter().position(
		        |x| { x == handle }).is_some() {
		    self.current_output = Some(
			    self.foreground_executor.current_screen
				    .run_as_background(self.current_output.clone().unwrap(), 
				    terminal, gpio_pins).await
			);
		}
	}
}