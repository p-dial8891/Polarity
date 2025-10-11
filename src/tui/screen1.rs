use crate::tui::{Components, IntoComponent};
use std::rc::{Rc, Weak};
use crate::tui;
use std::ops::IndexMut;

type Component = tui::Component<Model,View,Controller>;

pub struct Screen1<'a> {

	v : Rc<Vec<Rc<Component>>>,
	a : &'a mut u32

}

#[derive(Clone)]
pub struct Controller {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

#[derive(Clone)]
pub struct Model {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

#[derive(Clone)]
pub struct View {
	
	pub w : Weak<Vec<Rc<Component>>>,
	
}

impl Screen1<'_> {
			
	pub fn controller(&self) -> Rc<Component> {
		
		self.v[0].clone()
	
	}
	
}

impl Controller {

	pub fn model(&self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&self) -> Rc<Component> {

        let p = self.w.upgrade().unwrap();
        p[1].clone()
	
    }

}

impl Model {

	pub fn view(&self) -> Rc<Component> {
		
	    self.step()
		
	}

	fn step(&self) -> Rc<Component> {

        let p = self.w.upgrade().unwrap();
        p[2].clone()
	
    }

}

impl View {

	pub fn end(&self) -> () {
		
	}

}

impl<'c> Components<'c> for Screen1<'_> {
	type Item<'b> = Screen1<'b>;

	fn new(data: &'c mut u32) -> Screen1<'c> {
		
		Screen1 {
			v : Rc::new_cyclic(|wp| {
				
				let mut n = Vec::new();
			    n.insert(0, Rc::new(Component::Controller(Controller { 
					w: wp.clone()  } )  ) 
				);
				
				n.insert(1, Rc::new(Component::Model(Model { 
					w: wp.clone()  } ) )
				);	
				
				n.insert(2, Rc::new(Component::View(View { 
					w: wp.clone()  } ) )
			    ); n } ),
			a : data
		}
	
	}

}

impl
IntoComponent<Model,View,Controller> for Component {
	
	fn unwrap_controller(&self) -> &Controller {
		
		match self {
			
			Component::Controller(c) => c,
            _                        => panic!("Wrong type"),
		
		}
		
    }

	fn unwrap_model(&self) -> &Model {
		
		match self {
			
			Component::Model(m) => m,
            _                   => panic!("Wrong type"),
		
		}
		
    }
	
	fn unwrap_view(&self) -> &View {
		
		match self {
			
			Component::View(v) => v,
            _                  => panic!("Wrong type"),
		
		}
		
    }	

}