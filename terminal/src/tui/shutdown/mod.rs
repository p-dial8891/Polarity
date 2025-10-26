use crate::tui;
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Execute};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};

mod controller;
mod model;
mod view;

use crate::tui::shutdown::{
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
pub type Executor<'c> = tui::Execute<'c, Shutdown>;

pub struct Shutdown {
    pub v : Vec<State>
}

impl<'c> Components<'c> for Shutdown {
    type Item = Shutdown;
    type Output = Output;

    fn new() -> Shutdown {
        Shutdown {
            v: Vec::from([
                State::Controller(ControllerState { 
				    start: true}),
                State::Model(ModelState	{_a:()}),
                State::View(ViewState {_a:()}),
            ])
        }
    }

    async fn run(
        &mut self,
        o: Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 5],
    ) -> Output {
        o.unwrap_controller()
            .compute(&mut self.v[0], terminal, gpio_pins).await
            .unwrap_model()
            .compute(&mut self.v[1], terminal, gpio_pins).await
            .unwrap_view()
            .compute(&mut self.v[2], terminal, gpio_pins).await
    }

    async fn start(&mut self) -> Output {
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
pub enum ControllerCommand {
    Noop,
    Init
}

#[derive(Clone)]
pub enum ModelCommand {
    Noop,
    Init
}

#[derive(Clone)]
pub enum ViewCommand {
    Noop,
    Init
}

impl<'c> Execute<'c,Shutdown> {
	pub async fn init(&mut self, handle: &String) {
		if handle == &self.screen_name {
		    self.current_output = Some(self.current_screen.start().await);
		}
	}
	
	pub async fn execute(
	    &mut self, 
		handle: &String,
		terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 5]
	) {
		if handle == &self.screen_name {
		    self.current_output = Some(
			    self.current_screen.run(self.current_output.clone().unwrap(), 
				terminal, gpio_pins).await
			);
		}
	}
}
