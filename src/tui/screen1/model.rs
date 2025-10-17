use crate::tui;
use crate::tui::screen1::{controller::Controller, view::View};
use crate::tui::{Components, Compute, IntoComponent};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
//type State = tui::ComponentData<Model, View, Controller>;
//type Output = tui::ComponentData<Model, View, Controller>;
use crate::tui::screen1::{State, Output};
use crate::polaris::polarisHandle;

#[derive(Clone)]
pub struct Model {
    pub data: polarisHandle
}

#[derive(Clone)]
pub struct ModelState {
    pub s: u32,
    pub b: u16,
}

impl<'c> Compute<'c, Model, View, Controller> for Model {
    type State = State;
    type Output = Output;

    fn compute(
	    self, s: &mut State, 
		_: &mut DefaultTerminal, 
		_: [&'c InputPin; 6]
	) -> Output {
        Output::View(View { data : self.data })
    }
}
