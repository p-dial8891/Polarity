use crate::tui;
use crate::tui::screen1::{controller::Controller, model::Model,
    ViewCommand::{self}};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Clone)]
pub struct View {
    pub data: polarisHandle,
	pub cmd: ViewCommand,
}

pub struct ViewState {
    pub s: u16,
    pub b: u64,
	pub tx : Sender<Option<()>>
}


impl<'c> Compute<'c> for View {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: [&'c InputPin; 6],
    ) -> Output {
        Output::Controller(Controller { data : self.data })
    }
}
