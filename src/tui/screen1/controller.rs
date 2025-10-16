use crate::tui::{Components, IntoComponent, Compute};
use crate::tui;
use crate::tui::screen1::{model::Model,
    view::View};

type State = tui::ComponentData<Model,View,Controller>;
type Output = tui::ComponentData<Model,View,Controller>;


#[derive(Clone)]
pub struct Controller {
	
	pub s : i32,
	pub b : u8
	
}

impl Compute<Model,View,Controller> for Controller {
	type State = State;
	type Output = Output;
	
	fn compute(self, s: &mut State) -> Output {	
		  Output::Model( Model {
	        s: 32567,
		    b: 64 } ) 
    }
}
