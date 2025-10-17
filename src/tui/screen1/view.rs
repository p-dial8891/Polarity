use crate::tui;
use crate::tui::screen1::{controller::Controller, model::Model};
use crate::tui::{Components, Compute, IntoComponent};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
//type State = tui::ComponentData<Model, View, Controller>;
//type Output = tui::ComponentData<Model, View, Controller>;
use crate::tui::screen1::{State, Output};
use crate::polaris::polarisHandle;

#[derive(Clone)]
pub struct View {
    pub data: polarisHandle
}

#[derive(Clone)]
pub struct ViewState {
    pub s: u16,
    pub b: u64,
}


impl<'c> Compute<'c, Model, View, Controller> for View {
    type State = State;
    type Output = Output;

    fn compute(
        self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: [&'c InputPin; 6],
    ) -> Output {
        Output::Controller(Controller { data : self.data })
    }
}
