use crate::tui;
use crate::tui::screen1::{controller::Controller, model::Model,
    ViewCommand::{self, Draw, PlayTrack}};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use rppal::gpio::{self, InputPin};
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::collections::HashSet;

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

#[derive(Clone)]
pub struct View {
    pub data: polarisHandle,
	pub cmd: ViewCommand,
}

pub struct ViewState {
    pub s: u16,
    pub b: u64,
	pub tx : Sender<Option<()>>
}


/// Render the UI with various lists.
fn render(
    frame: &mut Frame,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    toggle_play: bool,
    l_playlist: &HashSet<usize>,
) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(8), Length(2)]);
    let [top, bottom] = vertical.areas(frame.area());

    //let title = Line::from_iter([
    //    Span::from("List Widget").bold(),
    //    Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    //]);
    //frame.render_widget(title.centered(), top);

    render_list(frame, top, list_state, list_model, l_playlist);
    render_bottom(frame, bottom, toggle_play);
}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(
    frame: &mut Frame,
    area: Rect,
    list_state: &mut ListState,
    list_model: &Vec<String>,
    l_playlist: &HashSet<usize>,
) {
    let list =
        List::new(list_model.into_iter().map(|x| x.as_str()).enumerate().map(
            |(i, x)| {
                if l_playlist.contains(&i) {
                    ListItem::new(x).yellow()
                } else {
                    ListItem::new(x).white()
                }
            },
        ))
        //    .highlight_style(SELECTED_STYLE);
        .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_bottom(frame: &mut Frame, area: Rect, auto_play: bool) {
    let final_text;
    match auto_play {
        false => final_text = String::from("\n             "),
        true => {
            let mut temp_text = String::from("\n             ");
            temp_text.extend(["A"]);
            final_text = temp_text;
        }
    }
    let text = Paragraph::new(final_text);
    frame.render_widget(text, area);
}

impl<'c> Compute<'c> for View {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        terminal: &mut DefaultTerminal,
        _: [&'c InputPin; 6],
    ) -> Output {
		
		match self.cmd {
		    PlayTrack(name, data, mut list_state, playlist) => {
                terminal.draw(|frame| {
				    render(frame, &mut list_state, &data, false, &playlist) } ).unwrap();
            },
			
			Draw(data, mut list_state, playlist) => {
                terminal.draw(|frame| {
				    render(frame, &mut list_state, &data, false, &playlist) } ).unwrap();
            },

            _ => {}			
		}
		
		Output::Controller(Controller { data : self.data })
    }
}
