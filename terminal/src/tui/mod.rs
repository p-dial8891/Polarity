pub mod app;
mod screen1;
mod shutdown;
mod playback;
mod input;

use ratatui::{DefaultTerminal, Frame};
use input::Input;
use std::rc::Rc;
use ratatui::layout::{Rect};

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
        gpio_pins: &mut Input,
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
        gpio_pins: &mut Input,
    ) -> Self::Output;
}

trait Render<S> {
	
	fn renderer<'a>(state : &'a mut S) -> 
	    Box<dyn FnOnce(&mut Frame<'a>, Rect) -> () +'_>;
}

struct Execute<'c, S : Components<'c>> {
	
	screen_names : Vec<String>,
	current_output : Option<S::Output>,
	current_screen : S,
}

struct ExecuteLayout1<C_Top,C_Bottom,R_Top,R_Bottom>  {

	controllers : (Option<C_Top>,Option<C_Bottom>),
	renderers : (R_Top, R_Bottom)

}

struct ExecuteBG<'c, S : Components<'c>, BG> {
	
	pub foreground_executor : &'c mut Execute<'c, S>,
	current_output : Option<BG>,
	
}

struct App_List(Vec<String>);

impl App_List {
	
    pub fn register(& mut self, name : &str ) -> String {
	    let temp = String::from(name);
		self.0.push(temp.clone());
	    temp
    }
	
	pub fn get_iter(&self) -> impl Iterator<Item=&String> {
		self.0.iter().cycle()
	}
}