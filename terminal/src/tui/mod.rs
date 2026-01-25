pub mod app;
mod screen1;
mod shutdown;
mod playback;
mod input;

use ratatui::{DefaultTerminal, Frame};
use input::Input;
use std::rc::Rc;
use ratatui::layout::{Constraint, Layout, Rect};

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

pub async fn run_screen<'c,S,T,C,M,V> (
    c: T,
	s: &'c mut S,
	terminal: &mut DefaultTerminal,
	gpio_pins: &mut Input,
) -> T
where
    T: IntoComponent<M,V,C>,
    C: Compute<'c, State=S, Output=T>,
    M: Compute<'c, State=S, Output=T>,
    V: Compute<'c, State=S, Output=T>,
{
	c
    .unwrap_controller()
	.compute(s, terminal, gpio_pins).await
	.unwrap_model()
	.compute(s, terminal, gpio_pins).await
	.unwrap_view()
	.compute(s, terminal, gpio_pins).await
}

trait Render<S> {
	
	fn renderer(state : & mut S) -> 
	    impl FnOnce(&mut Frame, Rect) -> ();

    fn redraw(&self) -> bool {
        true
    }
}

trait ExecutorForLayout1<S, T1, T2, M1, M2, V1, V2, C1, C2> 
  where 
        T1 : IntoComponent<M1,V1,C1> + Clone,
        T2 : IntoComponent<M2,V2,C2> + Clone,
        C1 : Render<S> + for<'c> Compute<'c, State=S, Output=T1>,
        C2 : Render<S> + for<'c> Compute<'c, State=S, Output=T2>,
        M1 : for<'c> Compute<'c, State=S, Output=T1>,
        M2 : for<'c> Compute<'c, State=S, Output=T2>,
        V1 : for<'c> Compute<'c, State=S, Output=T1>,
        V2 : for<'c> Compute<'c, State=S, Output=T2>,
{

    fn get_controllers(&self) -> (T1, T2);

    fn set_controllers(&mut self, controllers : (T1, T2));

    async fn init(&mut self);

    async fn execute(
        &mut self,
        state: &mut S,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) {
 
        let controllers = self.get_controllers();

        let c1 = run_screen(controllers.0, state, terminal, gpio_pins).await;
        let c2 = run_screen(controllers.1, state, terminal, gpio_pins).await;

        self.set_controllers((c1,c2));

        let controllers = self.get_controllers();

        let r_top = controllers.0.unwrap_controller().redraw();
        let r_bottom = controllers.1.unwrap_controller().redraw();

        if !r_top && !r_bottom {
            return;
        }

        terminal.draw( |frame| {
            use Constraint::{Fill, Length, Min};
            let vertical = Layout::vertical([Fill(1), Length(2)]);
            let [top, bottom] = vertical.areas(frame.area());

            //render_top(frame, top);
            let r = C1::renderer(state);
            r(frame, top);
            //render_list(frame, bottom, &mut self.screen.v.selection);
            let r = C2::renderer(state);
            r(frame, bottom);
		}).unwrap();
    }

}

trait ExecutorForLayout2<S, T1, T2, M1, M2, V1, V2, C1, C2> 
  where 
        T1 : IntoComponent<M1,V1,C1> + Clone,
        T2 : IntoComponent<M2,V2,C2> + Clone,
        C1 : Render<S> + for<'c> Compute<'c, State=S, Output=T1>,
        C2 : Render<S> + for<'c> Compute<'c, State=S, Output=T2>,
        M1 : for<'c> Compute<'c, State=S, Output=T1>,
        M2 : for<'c> Compute<'c, State=S, Output=T2>,
        V1 : for<'c> Compute<'c, State=S, Output=T1>,
        V2 : for<'c> Compute<'c, State=S, Output=T2>,
{

    fn get_controllers(&self) -> (T1, T2);

    fn set_controllers(&mut self, controllers : (T1, T2));

    async fn init(&mut self);

    async fn execute(
        &mut self,
        state: &mut S,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) {
 
        let controllers = self.get_controllers();

        let c1 = run_screen(controllers.0, state, terminal, gpio_pins).await;
        let c2 = run_screen(controllers.1, state, terminal, gpio_pins).await;

        self.set_controllers((c1,c2));

        let controllers = self.get_controllers();

        let r_top = controllers.0.unwrap_controller().redraw();
        let r_bottom = controllers.1.unwrap_controller().redraw();

        if !r_top && !r_bottom {
            return;
        }

        terminal.draw( |frame| {
            use Constraint::{Fill, Length, Min};
            let vertical = Layout::vertical([Length(2), Fill(1)]);
            let [top, bottom] = vertical.areas(frame.area());

            //render_top(frame, top);
            let r = C1::renderer(state);
            r(frame, top);
            //render_list(frame, bottom, &mut self.screen.v.selection);
            let r = C2::renderer(state);
            r(frame, bottom);
		}).unwrap();
    }

}


trait ExecutorForBackground<S, T, M, V, C> 
  where 
        T : IntoComponent<M,V,C> + Clone,
        C : for<'c> Compute<'c, State=S, Output=T>,
        M : for<'c> Compute<'c, State=S, Output=T>,
        V : for<'c> Compute<'c, State=S, Output=T>,
{

    fn get_controller(&self) -> T;

    fn set_controller(&mut self, controller : T);

    async fn init(&mut self);

    async fn execute(
        &mut self,
        state: &mut S,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) {
 
        let controller = self.get_controller();

        let c = run_screen(controller, state, terminal, gpio_pins).await;

        self.set_controller(c);

    }

}

struct Execute<'c, S : Components<'c>> {
	
	screen_names : Vec<String>,
	current_output : Option<S::Output>,
	current_screen : S,
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