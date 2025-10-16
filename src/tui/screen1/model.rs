use crate::tui::{Components, IntoComponent, Compute};
use crate::tui;
use crate::tui::screen1::{controller::Controller,
    view::View};

type State = tui::ComponentData<Model,View,Controller>;
type Output = tui::ComponentData<Model,View,Controller>;

#[derive(Clone)]
pub struct Model {
	
	pub s : u32,
	pub b : u16
	
}

impl Compute<Model,View,Controller> for Model {
	type State = State;
	type Output = Output;
	
	fn compute(self, s: &mut State) -> Output {			
		  Output::View ( View {
	        s: 32567,
		    b: 64 } )
    }
}
