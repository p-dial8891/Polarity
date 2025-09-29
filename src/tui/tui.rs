use crate::ComponentId::{C1, C2};
use crate::ComponentList::{L1, L2};
use std::rc::Rc;

// Enums
#[derive(Clone)]
enum ComponentList {
	L1(Component1Controller),
	L2(Component2Controller)
}

#[derive(Clone)]
enum ComponentId {
	C1(usize),
	C2(usize)
}

// Traits

trait Controller {
	fn step(&self) -> Option<Rc<dyn Model>>;
}

trait Model {
	fn step(&self) -> Option<Rc<dyn View>>;
}

trait View {
	fn end(&self);
}

// Component 1
#[derive(Clone)]
struct Component1Controller {
	id: ComponentId,
	a: i32
}
#[derive(Clone)]
struct Component1Model {
	id: ComponentId,
	b: String
}
#[derive(Clone)]
struct Component1View {
	id: ComponentId,
	c: i8
}

// Implementations - Component 1
impl Controller for Component1Controller {
	fn step(&self) -> Option<Rc<dyn Model>>{
        if let C1(i) = self.id { 
			let c1_mdl = Component1Model { 
			  id: C1(0),
			  b: String::from("Hello")
			};
		    Some(Rc::new(c1_mdl)) 
		} else {
		    None
		}
	}
}

impl Model for Component1Model {
	fn step(&self) -> Option<Rc<dyn View>> {
        if let C1(i) = self.id { 
			let c1_viw = Component1View { 
			  id: C1(0),
			  c: 2
			};
		    Some(Rc::new(c1_viw)) 
		} else {
		    None
		}
	}
}

impl View for Component1View {
	fn end(&self){

	}
}

// Component 2
#[derive(Clone)]
struct Component2Controller {
	id: ComponentId,
	a: i32
}
#[derive(Clone)]
struct Component2Model {
	id: ComponentId,
	b: String
}
#[derive(Clone)]
struct Component2View {
	id: ComponentId,
	c: i8
}


// Implementations - Component 2
impl Controller for Component2Controller {
	fn step(&self) -> Option<Rc<dyn Model>> {
        if let C2(i) = self.id { 
			let c2_mdl = Component2Model { 
			  id: C2(1),
			  b: String::from("GoodBye")
			};
		    Some(Rc::new(c2_mdl)) 
		} else {
			None
		}
	}
}

impl Model for Component2Model {
	fn step(&self) -> Option<Rc<dyn View>> {
        if let C2(i) = self.id { 
			let c2_viw = Component2View { 
			  id: C2(1),
			  c: 4
			};
		    Some(Rc::new(c2_viw)) 
		} else {
			None
		}
	}
}

impl View for Component2View {
	fn end(&self){

	}
}

fn execute<C: Controller>(controller: C) {
	controller
	    .step().unwrap()
		.step().unwrap()
		.end()
}

// Main application
fn main() {
    let mut v_ctl = Vec::<ComponentList>::new();

    let c1_ctl = Component1Controller { 
	  id: C1(0),
	  a: 32
	};

    let c2_ctl = Component2Controller { 
	  id: C2(1),
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