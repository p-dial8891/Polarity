use crate::tui::{
	self, screen1, screen1::Screen1, 
	Components, IntoComponent, Compute };


pub fn main() {
	let mut d = 34;
	
    let mut s = Screen1::new(&mut d);

    s.run();
}