use crate::tui;
use crate::tui::playback::{title::model::Model, title::view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp, Render};
use crate::tui::input::Input;
use crate::tui::playback::{State, Output1 as Output};
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
	pub redraw : bool
}

impl Compute for Controller {
    type State = State;
    type Output = Output;

    async fn compute(
        mut self,
        s: &mut State,
        _: &mut DefaultTerminal,
        input: &mut Input,
    ) -> Output {
		
		match self.cmd {
			
			ControllerCommand::Init => {
				Output::Model(Model { cmd : ModelCommand::Init	})
			}
			
			ControllerCommand::Noop => {
				Output::Model(Model { cmd : ModelCommand::Noop	})				
			}
		}
	}
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init,
			redraw: true,
		}
		
	}
	
}


/// Render a bottom-to-top list.
pub fn render_top<'a>(frame: &mut Frame<'a>, area: Rect) {

    let text = Paragraph::new(String::from("  Playback\n"));
    frame.render_widget(text, area);
}

impl Render<State> for Controller {

    fn renderer(state : &mut State) -> 
	    impl FnOnce(&mut Frame, Rect) -> () {

        move |f,r| { render_top( f, r ); }
		
    }

	fn redraw(&self) -> bool {
		self.redraw
	}
}

