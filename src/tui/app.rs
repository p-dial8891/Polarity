use rppal::gpio::Gpio;
use std::boxed::Box;
use std::sync::LazyLock;
use crate::tui;
use crate::tui::{ComponentList, ScreenList};
use crate::tui::ComponentList::{L1, L2};
use crate::tui::screen1;
use crate::tui::Controller;

fn execute<C: Controller>(mut controller: C) {
	let mut model = controller.step().unwrap();
	let model_mut = model.as_mut();
	let mut view  = model_mut.step().unwrap();
	let view_mut  = view.as_mut();
	let _         = view_mut.end();
}

#[derive(Clone)]
pub struct Env {
	gpio_device: &'static Gpio,
	pub active_screen: ScreenList
}

static gpio_d_ll: LazyLock<Gpio> = LazyLock::new(|| {
Gpio::new().unwrap() } );

// Main application
pub fn main() {
    let mut v_ctl = Vec::<ComponentList>::new();
    let gpio_d = &*gpio_d_ll;

    let c1_ctl = screen1::Controller { 
	  env: Env { gpio_device: gpio_d,
          active_screen: ScreenList::S1	  },
	  a: 32
	};

	v_ctl.insert(0,L1(c1_ctl));

    for i in v_ctl {
		match i {
			L1(list_item) => { execute(list_item); }
            L2(list_item) => { execute(list_item); }
		}
	}
}