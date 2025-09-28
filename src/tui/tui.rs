use crate::ComponentId::{C1, C2};
use std::rc::Rc;

// Enums
#[derive(Clone)]
enum ComponentId {
	C1(usize),
	C2(usize)
}

// Traits

trait Controller {
    fn register(&self, v: &mut Vec<Rc<dyn Controller>>);
	fn step(&self) -> Option<Rc<dyn Model>>;
}

trait Model {
    fn register(&self, v: &mut Vec<Rc<dyn Model>>);
	fn step(&self) -> Option<Rc<dyn View>>;
}

trait View {
    fn register(&self, v: &mut Vec<Rc<dyn View>>);
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
    fn register(&self, v: &mut Vec<Rc<dyn Controller>>) {
        if let C1(i) = self.id { 
		  v.insert(i,Rc::new((*self).clone())); 
		}		
	}
	
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
    fn register(&self, v: &mut Vec<Rc<dyn Model>>) {
        if let C1(i) = self.id { 
		  v.insert(i,Rc::new((*self).clone())); 
		}
	}
	
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
    fn register(&self, v: &mut Vec<Rc<dyn View>>) {
        if let C1(i) = self.id { 
		  v.insert(i, Rc::new((*self).clone())); 
		}		
	}
	
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
    fn register(&self, v: &mut Vec<Rc<dyn Controller>>) {
        if let C2(i) = self.id { 
		  v.insert(i,Rc::new((*self).clone())); 
		}		
	}
	
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
    fn register(&self, v: &mut Vec<Rc<dyn Model>>) {
        if let C2(i) = self.id { 
		  v.insert(i,Rc::new((*self).clone())); 
		}		
	}
	
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
    fn register(&self, v: &mut Vec<Rc<dyn View>>) {
        if let C2(i) = self.id { 
		  v.insert(i,Rc::new((*self).clone())); 
		}		
	}
	
	fn end(&self){

	}
}

// Main application
fn main() {
    let mut v_ctl = Vec::<Rc<dyn Controller>>::new();

    let c1_ctl = Component1Controller { 
	  id: C1(0),
	  a: 32
	};

    let c2_ctl = Component2Controller { 
	  id: C2(1),
	  a: 28
	};

	c1_ctl.register(&mut v_ctl);
	c2_ctl.register(&mut v_ctl);


	c1_ctl
	  .step().unwrap()
	  .step().unwrap()
	  .end();
	  
	c2_ctl
	  .step().unwrap()
	  .step().unwrap()
	  .end();
	  
}