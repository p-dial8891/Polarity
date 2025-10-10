use crate::tui::{Components};
use std::rc::{Rc, Weak};
use crate::tui;

type Component = tui::Component<Model,View,Controller>;

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

impl Screen1 {
			
	pub fn controller(&mut self) -> Rc<Component> {
		
		self.v[0].clone()
	
	}
	
}

impl Controller {

	pub fn model(&mut self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&mut self) -> Rc<Component> {
		
		let p = self.w.upgrade().unwrap();
        p[1].clone()
	
    }

}

impl Model {

	pub fn view(&mut self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&mut self) -> Rc<Component> {
		
		let p = self.w.upgrade().unwrap();
        p[2].clone()
	
    }

}

impl View {

	pub fn end(&mut self) -> () {
		
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

impl Component {
	
	pub fn unwrap_controller(&mut self) -> &mut Controller {
		
		match self {
			
			Component::Controller(c) => c,
            _                        => panic!("Wrong type"),
		
		}
		
    }

	pub fn unwrap_model(&mut self) -> &mut Model {
		
		match self {
			
			Component::Model(m) => m,
            _                   => panic!("Wrong type"),
		
		}
		
    }
	
	pub fn unwrap_view(&mut self) -> &mut View {
		
		match self {
			
			Component::View(v) => v,
            _                  => panic!("Wrong type"),
		
		}
		
    }	
}