pub mod app;
mod screen1;
mod shutdown;

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

    async fn start(&mut self) -> Self::Output;

    async fn run(
        &mut self,
        o: Self::Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 5],
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
        gpio_pins: [&'c InputPin; 5],
    ) -> Self::Output;
}

struct Execute<'c, S : Components<'c>> {
	
	screen_name : String,
	current_output : Option<S::Output>,
	current_screen : S,
	
}

struct App_List(Vec<String>);

impl App_List {
	
    pub fn enumerate(& mut self, name : &str ) -> String {
	    let temp = String::from(name);
		self.0.push(temp.clone());
	    temp
    }
	
	pub fn get_iter(&self) -> impl Iterator<Item=&String> {
		self.0.iter().cycle()
	}
}