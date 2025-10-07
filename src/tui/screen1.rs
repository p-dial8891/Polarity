use crate::tui::Component;
use std::rc::{Rc, Weak};

pub struct Starting {

	pub w : Weak<Vec<Rc<dyn Component>>>,
	
}	
	
pub struct Controller {
	
	pub w : Weak<Vec<Rc<dyn Component>>>,
	
}

impl Component for Starting {

    fn controller(&mut self) -> Option<Rc<dyn Component>> {
		
		Some(self.step())
		
	}
	
	fn model(&mut self) -> Option<Rc<dyn Component>> {
		
		None
		
	}
	
	fn view(&mut self) {
		
	}

	fn step(&mut self) -> Rc<dyn Component> {
		
		let p = self.w.upgrade().unwrap();
        p[1].clone()
	
    }

}

impl Component for Controller {

    fn controller(&mut self) -> Option<Rc<dyn Component>> {
		
		None
		
	}
	
	fn model(&mut self) -> Option<Rc<dyn Component>> {
		
		None
		
	}
	
	fn view(&mut self) {
		
	}

	fn step(&mut self) -> Rc<dyn Component> {
		
		let p = self.w.upgrade().unwrap();
        p[1].clone()
	
    }

}
