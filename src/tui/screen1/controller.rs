use crate::tui;
use crate::tui::screen1::{model::Model, view::View};
use crate::tui::{Components, Compute, IntoComponent};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};

type State = tui::ComponentData<Model, View, Controller>;
type Output = tui::ComponentData<Model, View, Controller>;

#[derive(Clone)]
pub struct Controller {
    pub s: i32,
    pub b: u8,
}

impl<'c> Compute<'c, Model, View, Controller> for Controller {
    type State = State;
    type Output = Output;

    fn compute(
        self,
        s: &mut State,
        _: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Output {
        Output::Model(Model { s: 32567, b: 64 })
    }
}
