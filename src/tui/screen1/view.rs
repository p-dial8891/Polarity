use crate::tui::{Components, IntoComponent, Compute};
use crate::tui;
use crate::tui::screen1::{model::Model, controller::Controller};

type State = tui::ComponentData<Model,View,Controller>;
type Output = tui::ComponentData<Model,View,Controller>;

#[derive(Clone)]
pub struct View {
	
	pub s : u16,
	pub b : u64
	
}

impl Compute<Model,View,Controller> for View {
	type State = State;
	type Output = Output;
	
	fn compute(self, s: &mut State) -> Output {			
		  Output::Controller ( Controller {
	        s: 32567,
		    b: 7 } )
    }
}
		
		