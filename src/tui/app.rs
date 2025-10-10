use crate::tui::{self, screen1, Components};
use std::rc::Rc;

pub fn main() {
    let mut cs = screen1::Screen1::new();
	cs.initialise();
	
	let mut bind = cs.controller();
	let mut c = Rc::get_mut(&mut bind).unwrap()
	    .unwrap_controller();
		
	let mut bind = c.model();
	let mut m = Rc::get_mut(&mut bind).unwrap()
	    .unwrap_model();
		
	let mut bind = m.view();
	let mut v = Rc::get_mut(&mut bind).unwrap()
	    .unwrap_view();

	let _     = v.end();
	  
}