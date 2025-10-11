mod screen1;
pub mod app;

use std::rc::Rc;

#[derive(Clone)]
enum Component<M,V,C> {
	
	Controller(C),
	Model(M),
	View(V)
	
}

trait Components<'c> {
	type Item<'b>;
	
	fn new(data : &'c mut u32) -> Self::Item<'c>;

}

trait IntoComponent<M,V,C> {
	
	fn unwrap_controller(&self) -> &C;
	
	fn unwrap_model(&self) -> &M;
	
	fn unwrap_view(&self) -> &V;
	
}