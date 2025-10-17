use crate::tui;
use crate::tui::{Components, Compute, IntoComponent};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};

mod controller;
mod model;
mod view;

use crate::tui::screen1::{controller::Controller, model::Model, view::View};

type State = tui::ComponentData<Model, View, Controller>;
type Output = tui::ComponentData<Model, View, Controller>;

pub struct Screen1 {
    pub v: Vec<State>,
}

impl<'c> Components<'c> for Screen1 {
    type Item = Screen1;
    type Output = Output;

    fn new() -> Screen1 {
        Screen1 {
            v: Vec::from([
                State::Controller(Controller { s: 0, b: 0 }),
                State::Model(Model { s: 0, b: 0 }),
                State::View(View { s: 0, b: 0 }),
            ]),
        }
    }

    fn run(
        &mut self,
        o: Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Output {
        o.unwrap_controller()
            .compute(&mut self.v[0], terminal, gpio_pins)
            .unwrap_model()
            .compute(&mut self.v[1], terminal, gpio_pins)
            .unwrap_view()
            .compute(&mut self.v[2], terminal, gpio_pins)
    }
}

impl Screen1 {
    pub fn start(&self) -> Output {
        Output::Controller(Controller { s: 32567, b: 7 })
    }
}

impl IntoComponent<Model, View, Controller> for Output {
    fn unwrap_controller(self) -> Controller {
        match self {
            Output::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model {
        match self {
            Output::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View {
        match self {
            Output::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}
