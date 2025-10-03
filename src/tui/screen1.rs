use crate::tui::ScreenList;
use crate::tui::Controller as ControllerTrait;
use crate::tui::Model as ModelTrait;
use crate::tui::View as ViewTrait;
use crate::tui::app::Env;

// Component 1
#[derive(Clone)]
pub struct Controller {
	pub env: Env,
	pub a: i32
}
#[derive(Clone)]
struct Model {
	env: Env,
	b: String
}
#[derive(Clone)]
struct View {
	env: Env,
	c: i8
}

// Implementations - Component 1
impl ControllerTrait for Controller {
	fn step(&mut self) -> Option<Box<dyn ModelTrait>>{
		let model = Model { 
		  env: self.env.clone(),
		  b: String::from("Hello")
		};
		Some(Box::new(model)) 
	}
	
	fn set_screen(&mut self) {
		self.env.active_screen = ScreenList::S1;
	}
}

impl ModelTrait for Model {
	fn step(&mut self) -> Option<Box<dyn ViewTrait>> {
		let view = View { 
		  env: self.env.clone(),
		  c: 2
		};
		Some(Box::new(view)) 
	}
}

impl ViewTrait for View {
	fn end(&mut self){

	}
}
