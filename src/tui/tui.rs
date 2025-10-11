use rppal::gpio::Gpio;
use crate::ComponentList::{L1, L2};
use std::boxed::Box;

use std::sync::LazyLock;

// Enums
#[derive(Clone)]
enum ComponentList {
	L1(Component1Controller),
	L2(Component2Controller)
}

#[derive(Clone)]
enum ScreenList {
	S1,
	S2,
	S3
}

// Traits

trait Controller {
	fn step(&mut self) -> Option<Box<dyn Model>>;
	fn set_screen(&mut self) {
		
	}
}

trait Model {
	fn step(&mut self) -> Option<Box<dyn View>>;
}

trait View {
	fn end(&mut self);
}

// Component 1
#[derive(Clone)]
struct Component1Controller {
	env: Env,
	a: i32
}
#[derive(Clone)]
struct Component1Model {
	env: Env,
	b: String
}
#[derive(Clone)]
struct Component1View {
	env: Env,
	c: i8
}

// Implementations - Component 1
impl Controller for Component1Controller {
	fn step(&mut self) -> Option<Box<dyn Model>>{
		let c1_mdl = Component1Model { 
		  env: self.env.clone(),
		  b: String::from("Hello")
		};
		Some(Box::new(c1_mdl)) 
	}
	
	fn set_screen(&mut self) {
		self.env.active_screen = ScreenList::S3;
	}
}

impl Model for Component1Model {
	fn step(&mut self) -> Option<Box<dyn View>> {
		let c1_viw = Component1View { 
		  env: self.env.clone(),
		  c: 2
		};
		Some(Box::new(c1_viw)) 
	}
}

impl View for Component1View {
	fn end(&mut self){

	}
}

// Component 2
#[derive(Clone)]
struct Component2Controller {
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
impl Controller for Component2Controller {
	fn step(&mut self) -> Option<Box<dyn Model>> {
		let c2_mdl = Component2Model { 
		  b: String::from("GoodBye")
		};
		Some(Box::new(c2_mdl)) 
	}
	
}

impl Model for Component2Model {
	fn step(&mut self) -> Option<Box<dyn View>> {
		let c2_viw = Component2View { 
		  c: 4
		};
		Some(Box::new(c2_viw)) 
	}
}

impl View for Component2View {
	fn end(&mut self){

	}
}

fn execute<C: Controller>(mut controller: C) {
	let mut model = controller.step().unwrap();
	let model_mut = model.as_mut();
	let mut view  = model_mut.step().unwrap();
	let view_mut  = view.as_mut();
	let _         = view_mut.end();
}

#[derive(Clone)]
struct Env {
	gpio_device: &'static Gpio,
	active_screen: ScreenList
}

pub static gpio_d_ll: LazyLock<Gpio> = LazyLock::new(|| {
Gpio::new().unwrap() } );

// Main application
fn main() {
    let mut v_ctl = Vec::<ComponentList>::new();
    let gpio_d = &*gpio_d_ll;

    let c1_ctl = Component1Controller { 
	  env: Env { gpio_device: gpio_d,
          active_screen: ScreenList::S1	  },
	  a: 32
	};

    let c2_ctl = Component2Controller { 
	  a: 28
	};

	v_ctl.insert(0,ComponentList::L1(c1_ctl));
	v_ctl.insert(1,ComponentList::L2(c2_ctl));

    for i in v_ctl {
		match i {
			L1(list_item) => { execute(list_item); }
            L2(list_item) => { execute(list_item); }
		}
	}
}