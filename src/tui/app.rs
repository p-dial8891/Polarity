use crate::tui::{self, screen1, screen1::Screen1, Components, IntoComponent, Compute};
use std::rc::Rc;

pub fn main() {
	let mut d = 34;
	
    let mut s = Screen1::new(&mut d);

	let c = s.start();	
	
	let state_in = s.v.pop_front().unwrap();
	let (m, state_out) = c.unwrap_controller().compute(state_in);
	s.v.push_back(state_out);
	
	let state_in = s.v.pop_front().unwrap();
	let (v, state_out) = m.unwrap_model().compute(state_in);
	s.v.push_back(state_out);
	
	let state_in = s.v.pop_front().unwrap();
	let (c, state_out) = v.unwrap_view().compute(state_in);
	s.v.push_back(state_out);
}