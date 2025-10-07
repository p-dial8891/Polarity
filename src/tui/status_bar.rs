use crate::tui::ScreenList;
use crate::tui::Controller as ControllerTrait;
use crate::tui::Model as ModelTrait;
use crate::tui::View as ViewTrait;

use std::rc::Rc;

// Component 2
#[derive(Clone)]
pub struct Component2Controller {
	a: i32
}
#[derive(Clone)]
struct Component2Model {
	b: String
}
#[derive(Clone)]
struct Component2View {
	c: i8
}


// Implementations - Component 2
impl ControllerTrait for Component2Controller {
	fn step(&mut self) -> Option<Rc<dyn ModelTrait>> {
		let c2_mdl = Component2Model { 
		  b: String::from("GoodBye")
		};
		Some(Rc::new(c2_mdl)) 
	}
	
}

impl ModelTrait for Component2Model {
	fn step(&mut self) -> Option<Rc<dyn ViewTrait>> {
		let c2_viw = Component2View { 
		  c: 4
		};
		Some(Rc::new(c2_viw)) 
	}
}

impl ViewTrait for Component2View {
	fn end(&mut self){

	}
}