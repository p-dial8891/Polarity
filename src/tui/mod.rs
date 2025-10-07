mod screen1;
pub mod app;

use std::rc::Rc;

struct Components {

	v : Rc<Vec<Rc<dyn Component>>>

}

trait Component {
	
	fn controller(&mut self) -> Option<Rc<dyn Component>>;
	
	fn model(&mut self) -> Option<Rc<dyn Component>>;
	
	fn view(&mut self) -> ();
	
	fn step(&mut self) -> Rc<dyn Component>;
	
}

impl Components {
	
	fn new() -> Self {
		
		Components {
			v : Rc::new(Vec::new())
		}
	
	}

    fn initialise(&mut self) {
		
	    let v2 = self.v.clone();
		let mut v_mut = Rc::get_mut(&mut self.v).unwrap();
		v_mut.insert(0, Rc::new(screen1::Starting { 
		    w: Rc::downgrade(&v2)  } ) 
		);
		
		v_mut.insert(1, Rc::new(screen1::Controller { 
		    w: Rc::downgrade(&v2)} )
		);	
	}
	
}

