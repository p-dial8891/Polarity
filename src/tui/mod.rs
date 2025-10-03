mod screen1;
mod status_bar;
pub mod app;

// Enums
#[derive(Clone)]
pub enum ComponentList {
	L1(screen1::Controller),
	L2(status_bar::Component2Controller)
}

#[derive(Clone)]
pub enum ScreenList {
	S1
}

// Traits
trait Controller {
	fn step(&mut self) -> Option<Box<dyn Model>>;
	fn set_screen(&mut self) {
		
	}
}

trait Model {
	fn step(&mut self) -> Option<Box<dyn View>>;
}

trait View {
	fn end(&mut self);
}