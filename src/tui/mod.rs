mod screen1;
pub mod app;

use std::rc::Rc;

enum Component<M,V,C> {
	
	Controller(C),
	Model(M),
	View(V)
	
}

trait Components {
	type Item;
	
	fn new() -> Self::Item;

    fn initialise(&mut self);
	
}
