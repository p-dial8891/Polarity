use crate::tui;
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Execute, ExecuteLayout1, run_screen, Render };
use crate::tui::input::Input;
use crate::tui::playback::title::view::render_top;
use crate::tui::playback::content::view::render_list;
use std::rc::Rc;
use std::sync::mpsc::{channel};
use std::collections::VecDeque;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

mod title;
mod content;

use crate::tui::playback::title::{
	controller::{
		Controller as Controller1
	}, 
	model::{
		Model as Model1, ComponentState
	}, 
	view::{
		View as View1
	}
};


pub struct Playback {
    pub v: State,
}

impl<'c> Components<'c> for Playback {
    type Item = Playback;
    type Output = Output1;

    fn new() -> Playback {
        Playback {
            v: ComponentState { 
				start: true, 
				selection: ListState::default().with_selected(Some(0))
            }
        }
    }

    async fn run(
        &mut self,
        o: Self::Output,
        terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input,
    ) -> Self::Output {
        o.unwrap_controller()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_model()
            .compute(&mut self.v, terminal, gpio_pins).await
            .unwrap_view()
            .compute(&mut self.v, terminal, gpio_pins).await
    }

    async fn start(&mut self) -> Self::Output {
        Self::Output::Controller ( 
		    Controller1::new().await
		)
    }
}

use crate::tui::playback::content::{
	controller::{
		Controller as Controller2, 
	}, 
	model::{
		Model as Model2, 
	}, 
	view::{
		View as View2, 
	}
};

pub type Output2 = tui::ComponentData<Model2, View2, Controller2>;

impl IntoComponent<Model1, View1, Controller1> for Output1 {
    fn unwrap_controller(self) -> Controller1 {
        match self {
            Output1::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model1 {
        match self {
            Output1::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View1 {
        match self {
            Output1::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

impl IntoComponent<Model2, View2, Controller2> for Output2 {
    fn unwrap_controller(self) -> Controller2 {
        match self {
            Output2::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(self) -> Model2 {
        match self {
            Output2::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(self) -> View2 {
        match self {
            Output2::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

impl IntoComp<Model1, View1, Controller1> for Output1 {
    fn unwrap_controller(&mut self) -> &mut Controller1 {
        match self {
            Output1::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(&mut self) -> &mut Model1 {
        match self {
            Output1::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(&mut self) -> &mut View1 {
        match self {
            Output1::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}

impl IntoComp<Model2, View2, Controller2> for Output2 {
    fn unwrap_controller(&mut self) -> &mut Controller2 {
        match self {
            Output2::Controller(c) => c,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_model(&mut self) -> &mut Model2 {
        match self {
            Output2::Model(m) => m,
            _ => panic!("Wrong type"),
        }
    }

    fn unwrap_view(&mut self) -> &mut View2 {
        match self {
            Output2::View(v) => v,
            _ => panic!("Wrong type"),
        }
    }
}
#[derive(Clone)]
pub enum ControllerCommand {
	
	Noop,
	Init,
	
}

#[derive(Clone)]
pub enum ModelCommand {
	
	Noop,
	Init,
    Req,
}

#[derive(Clone)]
pub enum ViewCommand {
	
	Noop,
    Init,
	Skip
}

pub type State = ComponentState;
pub type Output1 = tui::ComponentData<Model1, View1, Controller1>;
pub type Executor = tui::ExecuteLayout1<Playback,Output1,Output2>;

impl ExecuteLayout1<Playback,Output1,Output2> {
	
    pub async fn init(&mut self) {
		
		self.controllers.0 = Some( Output1::Controller( 
		    Controller1 { cmd : ControllerCommand::Init, redraw: true } ));
		self.controllers.1 = Some( Output2::Controller( 
            Controller2 { cmd : ControllerCommand::Init, redraw: true } ));
			
	}
	
	pub async fn execute(
	    &mut self, 
		terminal: &mut DefaultTerminal,
        gpio_pins: &mut Input
	) {

		self.controllers.0 = Some(run_screen(self.controllers.0.clone().unwrap(), &mut self.screen.v, terminal, gpio_pins).await);
        self.controllers.1 = Some(run_screen(self.controllers.1.clone().unwrap(), &mut self.screen.v, terminal, gpio_pins).await);

		let r_top = self.controllers.0.as_mut().unwrap().unwrap_controller().redraw;
		let r_bottom = self.controllers.1.as_mut().unwrap().unwrap_controller().redraw;
		
		if !r_top && !r_bottom {
			return;
		}

        terminal.draw(|frame| {
            use Constraint::{Fill, Length, Min};
            let vertical = Layout::vertical([Length(2), Length(8)]);
            let [top, bottom] = vertical.areas(frame.area());
            if r_top {		
                //render_top(frame, top);
                let r = View1::renderer(&mut self.screen.v);
                r(frame, top);
            }
            if r_bottom {
                //render_list(frame, bottom, &mut self.screen.v.selection);
                let r = View2::renderer(&mut self.screen.v);
                r(frame, bottom);
            }
		}).unwrap();

	}
}		
