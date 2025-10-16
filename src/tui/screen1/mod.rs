use crate::tui::{Components, IntoComponent, Compute};
use crate::tui;

mod model;
mod view;
mod controller;

use crate::tui::screen1::{model::Model, controller::Controller,
    view::View};

type State = tui::ComponentData<Model,View,Controller>;
type Output = tui::ComponentData<Model,View,Controller>;

pub struct Screen1<'a> {

	pub v : Vec<State>,
	pub a : &'a mut u32

}

impl<'c> Components<'c> for Screen1<'_> {
	type Item<'b> = Screen1<'b>;

	fn new(data: &'c mut u32) -> Screen1<'c> {
		
		Screen1 {
			v : Vec::from([
			        State::Controller( Controller { s:0, b:0 } ),
			        State::Model( Model { s:0, b:0 } ),
				    State::View( View { s:0, b:0 } )] ),
			a : data
		}
	
	}
	
	fn run(&mut self) {
		
		self.start()
		    .unwrap_controller().compute(&mut self.v[0])
			.unwrap_model().compute(&mut self.v[1])
			.unwrap_view().compute(&mut self.v[2]);

	}

}

impl Screen1<'_> {
	
	pub fn start(&self) -> Output {
 	    Output::Controller ( Controller {
	        s: 32567,
		    b: 7 } )
	}
}

impl
IntoComponent<Model,View,Controller> for Output {
	
	fn unwrap_controller(self) -> Controller {
		
		match self {
			
			Output::Controller(c) => c,
            _                       => panic!("Wrong type"),
		
		}
		
    }

	fn unwrap_model(self) -> Model {
		
		match self {
			
			Output::Model(m) => m,
            _                  => panic!("Wrong type"),
		
		}
		
    }
	
	fn unwrap_view(self) -> View {
		
		match self {
			
			Output::View(v) => v,
            _                 => panic!("Wrong type"),
		
		}
		
    }	

}

