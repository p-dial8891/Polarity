mod screen1;
pub mod app;

use std::rc::Rc;

enum Component {
	
	Controller(screen1::Controller),
	Model(screen1::Model),
	View(screen1::View)
	
}

trait Components {
	type Item;
	
	fn new() -> Self::Item;

    fn initialise(&mut self);
	
}

