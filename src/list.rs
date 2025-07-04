//! # [Ratatui] `List` example
//!
//! The latest version of this example is available in the [widget examples] folder in the
//! repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [widget examples]: https://github.com/ratatui/ratatui/blob/main/ratatui-widgets/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{DefaultTerminal, Frame};

use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

/// Run the application.
fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let gpio = Gpio::new().unwrap();
    let up = gpio.get(17).unwrap().into_input();
    let down = gpio.get(22).unwrap().into_input();
    let quit = gpio.get(5).unwrap().into_input();

    let mut list_state = ListState::default().with_selected(Some(0));
    loop {
        terminal.draw(|frame| render(frame, &mut list_state))?;
//        if let Some(key) = event::read()?.as_key_press_event() {
//            match key.code {
//                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
//                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
//                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
//                _ => {}
//            }
//        }

        if up.read() == 0.into() 
        { list_state.select_previous(); }
        
        if down.read() == 0.into()
        { list_state.select_next(); }

        if quit.read() == 0.into()
        { return Ok(()); }

        thread::sleep(Duration::from_millis(250));
    }
}

/// Render the UI with various lists.
fn render(frame: &mut Frame, list_state: &mut ListState) {
    let vertical = Layout::default();
    //let [top] = vertical.areas(frame.area());

    //let title = Line::from_iter([
    //    Span::from("List Widget").bold(),
    //    Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    //]);
    //frame.render_widget(title.centered(), top);

    render_list(frame, frame.area(), list_state);
    //render_bottom_list(frame, second);
}

/// Render a list.
pub fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let items = ["Item 1: Long title ", "Item 2: Very long title", "Item 3: title", "Item 4",
                 "Item 5: title",       "Item 6 : Very long title", "Item 7: Long title", 
                 "Item 8: Incredibly long title", "Item 9: Another title", "Item 10: Guess what?",
                 "Item 11: Incy wincy title", "Item 12: The last title?" ];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_bottom_list(frame: &mut Frame, area: Rect) {
    let items = [
        "[Remy]: I'm building one now.\nIt even supports multiline text!",
        "[Gusteau]: With enough passion, yes.",
        "[Remy]: But can anyone build a TUI in Rust?",
        "[Gusteau]: Anyone can cook!",
    ];
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Style::new().yellow().italic())
        .highlight_symbol("> ")
        .scroll_padding(1)
        .direction(ListDirection::BottomToTop)
        .repeat_highlight_symbol(true);

    let mut state = ListState::default();
    state.select_first();

    frame.render_stateful_widget(list, area, &mut state);
}
