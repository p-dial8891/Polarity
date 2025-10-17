pub mod app;
mod screen1;

use ratatui::DefaultTerminal;
use rppal::gpio::InputPin;
use std::rc::Rc;

#[derive(Clone)]
enum ComponentData<M, V, C> {
    Controller(C),
    Model(M),
    View(V),
}

trait Components<'c> {
    type Item;
    type Output;

    fn new() -> Self::Item;

    fn run(
        &mut self,
        o: Self::Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Self::Output;
}

trait IntoComponent<M, V, C> {
    fn unwrap_controller(self) -> C;

    fn unwrap_model(self) -> M;

    fn unwrap_view(self) -> V;
}

trait Compute<'c, M, V, C> {
    type State;
    type Output;

    fn compute(
        self,
        s: &mut Self::State,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Self::Output;
}
