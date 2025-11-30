use crate::tui;
use crate::tui::shutdown::{controller::Controller, model::Model,
    ViewCommand::{self, Init},
	ControllerCommand::{self, Noop}
};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use crate::tui::shutdown::{State, Output};

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use rppal::gpio::{self, InputPin};
use crate::tui::input::Input;

#[derive(Clone)]
pub struct View {
    pub cmd : ViewCommand
}

pub struct ViewState {
    pub _a : ()
}


/// Render the UI with various lists.
fn render(
    frame: &mut Frame,
    list_state: &mut ListState
) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(8), Length(2)]);
    let [top, bottom] = vertical.areas(frame.area());

    render_top(frame, top);
    render_list(frame, bottom, list_state);

}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
) {
    let list =
        List::new(["OK"])
        //.highlight_style(SELECTED_STYLE);
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_top(frame: &mut Frame, area: Rect) {

    let text = Paragraph::new(String::from("\n\n Shutdown ?\n\n"));
    frame.render_widget(text, area);
}

impl<'c> Compute<'c> for View {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: &mut Input,
    ) -> Output {
		
		match self.cmd {
			
			Init => {
				terminal.clear();	
				terminal.draw(|frame| {
					render(frame, &mut ListState::default().with_selected(Some(0)) ) }).unwrap();
			},
			
			_ => {}
		}
		
		Output::Controller(Controller { cmd : ControllerCommand::Noop })
    }
}
