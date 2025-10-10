use crate::tui::{Components, Component};
use std::rc::{Rc, Weak};

pub struct Screen1 {

	v : Rc<Vec<Rc<Component>>>

}

pub struct Controller {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

pub struct Model {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

pub struct View {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

impl Controller {

	fn model(&mut self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&mut self) -> Rc<Component> {
		
		let p = self.w.upgrade().unwrap();
        p[1].clone()
	
    }

}

impl Model {

	fn view(&mut self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&mut self) -> Rc<Component> {
		
		let p = self.w.upgrade().unwrap();
        p[2].clone()
	
    }

}

impl View {

	fn end(&mut self) -> () {
		
	}


}

impl Components for Screen1 {
	type Item = Screen1;

	fn new() -> Screen1 {
		
		Screen1 {
			v : Rc::new(Vec::new())
		}
	
	}

    fn initialise(&mut self) {
		
	    let v2 = self.v.clone();
		let mut v_mut = Rc::get_mut(&mut self.v).unwrap();
		v_mut.insert(0, Rc::new(Component::Controller(Controller { 
		    w: Rc::downgrade(&v2)  } ) ) 
		);
		
		v_mut.insert(1, Rc::new(Component::Model(Model { 
		    w: Rc::downgrade(&v2)} ) )
		);	
		
		v_mut.insert(2, Rc::new(Component::View(View { 
		    w: Rc::downgrade(&v2)} ) )
		);	
	}
	
}
