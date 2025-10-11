use crate::tui::{self, screen1::Screen1, Components, IntoComponent};
use std::rc::Rc;

pub fn main() {
	let mut d = 34;
	
    let s = Screen1::new(&mut d);
	
	let c = s.controller();
		
	let bind = c.unwrap_controller();
	let m    = bind.model();
		
	let bind = m.unwrap_model();
	let v    = bind.view();

	let bind = v.unwrap_view();
	let _    = bind.end();
	  
}