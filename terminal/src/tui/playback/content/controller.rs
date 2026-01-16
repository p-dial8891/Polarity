use crate::tui;
use crate::tui::playback::{content::model::Model, content::view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Render};
use crate::tui::input::Input;
use crate::tui::playback::{State, Output2 as Output};
use crate::tui::app::Keys::{self, *};
use std::process::Command;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};


#[derive(Clone)]
pub struct Controller {
    pub cmd : ControllerCommand,
	pub redraw : bool,
}


impl<'c> Compute<'c> for Controller {
    type State = State;
    type Output = Output;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Output {
		
		let mut state_data = s;
		
		match self.cmd {
			ControllerCommand::Init => {
				state_data.start = false;
				eprintln!("<Controller> : Initialised.");
				return Output::Model(Model { cmd : ModelCommand::Init });
			},
			_ => {}
		}
		
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { cmd : ModelCommand::Init });
		}

 		if input.read(REQ_KEY) == false {
			eprintln!("<Controller> : Req key pressed.");
			return Output::Model(Model { cmd : ModelCommand::Req });
		}
		
		Output::Model(Model { cmd : ModelCommand::Noop })
			
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init,
			redraw: true
		}
		
	}
	
}


const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
) {
    let list =
        List::new(["Skip"])
        //.highlight_style(SELECTED_STYLE);
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

impl Render<State> for Controller {

    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {

        move |f,r| { render_list( f, r, &mut state.selection ); }
		
    }
	
	fn redraw(&self) -> bool {
		self.redraw
	}
}
