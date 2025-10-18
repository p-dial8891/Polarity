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

    async fn run(
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

trait IntoComp<M, V, C> {
    fn unwrap_controller(&mut self) -> &mut C;

    fn unwrap_model(&mut self) -> &mut M;

    fn unwrap_view(&mut self) -> &mut V;
}

trait Compute<'c> {
    type State;
    type Output;

    async fn compute(
        self,
        s: &mut Self::State,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Self::Output;
}
