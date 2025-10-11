use crate::tui::{Components, IntoComponent, Compute};
use crate::tui;
use std::collections::VecDeque;

type S1_State = tui::ComponentData<Model,View,Controller>;
type S1_Output = tui::ComponentData<Model,View,Controller>;

pub struct Screen1<'a> {

	pub v : VecDeque<S1_State>,
	pub a : &'a mut u32

}

#[derive(Clone)]
pub struct Controller {
	
	pub s : i32,
	pub b : u8
	
}

#[derive(Clone)]
pub struct Model {
	
	pub s : u32,
	pub b : u16
	
}

#[derive(Clone)]
pub struct View {
	
	pub s : u16,
	pub b : u64
	
}

impl<'c> Components<'c> for Screen1<'_> {
	type Item<'b> = Screen1<'b>;

	fn new(data: &'c mut u32) -> Screen1<'c> {
		
		Screen1 {
			v : VecDeque::from([
			        S1_State::Controller( Controller { s:0, b:0 } ),
			        S1_State::Model( Model { s:0, b:0 } ),
				    S1_State::View( View { s:0, b:0 } )] ),
			a : data
		}
	
	}

}

impl Screen1<'_> {
	
	pub fn start(&self) -> S1_Output {
 	    S1_Output::Controller ( Controller {
	        s: 32567,
		    b: 7 } )
	}
}

impl
IntoComponent<Model,View,Controller> for S1_Output {
	
	fn unwrap_controller(self) -> Controller {
		
		match self {
			
			S1_Output::Controller(c) => c,
            _                       => panic!("Wrong type"),
		
		}
		
    }

	fn unwrap_model(self) -> Model {
		
		match self {
			
			S1_Output::Model(m) => m,
            _                  => panic!("Wrong type"),
		
		}
		
    }
	
	fn unwrap_view(self) -> View {
		
		match self {
			
			S1_Output::View(v) => v,
            _                 => panic!("Wrong type"),
		
		}
		
    }	

}

impl Compute<Model,View,Controller> for Controller {
	type State = S1_State;
	type Output = S1_Output;
	
	fn compute(self, s: S1_State) -> ( S1_Output, S1_State ) {			
	    ( 
		  S1_Output::Model( Model {
	        s: 32567,
		    b: 64 } ), 
	      S1_State::Controller(self)
	    )
    }
	
}

impl Compute<Model,View,Controller> for Model {
	type State = S1_State;
	type Output = S1_Output;
	
	fn compute(self, s: S1_State) -> ( S1_Output, S1_State ) {			
	    ( 
		  S1_Output::View ( View {
	        s: 32567,
		    b: 64 } ), 
	      S1_State::Model(self)
	    )
    }
	
}

impl Compute<Model,View,Controller> for View {
	type State = S1_State;
	type Output = S1_Output;
	
	fn compute(self, s: S1_State) -> ( S1_Output, S1_State ) {			
	    ( 
		  S1_Output::Controller ( Controller {
	        s: 32567,
		    b: 7 } ), 
	      S1_State::View(self)
	    )
    }
	
}
		
		