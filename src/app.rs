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
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{DefaultTerminal, Frame};

use std::thread;
use rppal::gpio::Gpio;

use std::time::Instant;
use std::process::Command;

use tokio::{time::sleep, task, net::TcpListener};
use tokio::io::AsyncReadExt;
use service::{PlayerClient, init_tracing};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::{net::SocketAddr, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Json};


mod polaris;

async fn sendRequestToPlayer(path: String)
{
    //init_tracing("Polarity example.");
    //println!("Polarity example");

    let mut transport = tarpc::serde_transport::tcp::connect(
        SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 102), 50051),
        Json::default,
    );
    transport.config_mut().max_frame_length(usize::MAX);
    let client = PlayerClient::new(client::Config::default(), transport.await.unwrap()).spawn();

    let mut cxt = context::current();
    cxt.deadline = Instant::now().checked_add(Duration::from_secs(60*5)).unwrap();
    let result = client.play(cxt, path).await
        .unwrap();
    //println!("{result}");
    
    sleep(Duration::from_millis(10)).await;
}

async fn listenerTask()
{
    let listener = TcpListener::bind("192.168.1.104:9000").await.unwrap();
    let (mut socket,_) = listener.accept().await.unwrap();
    let mut buf = [0; 1];
    socket.read(&mut buf).await.unwrap();
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().unwrap();
    let terminal = ratatui::init();
    let result = run(terminal).await;
    ratatui::restore();
    //result
    Ok(())
}

/// Run the application.
async fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let gpio = Gpio::new().unwrap();
    let up = gpio.get(17).unwrap().into_input();
    let down = gpio.get(22).unwrap().into_input();
    let quit = gpio.get(5).unwrap().into_input();
    let req  = gpio.get(6).unwrap().into_input();

    let track_data = polaris::getBody().await.unwrap();
    let mut taskHandle: Option<task::JoinHandle<()>> = None;
//    let list_model = polaris::polaris().await.map(|x| { x.0 }).collect::<Vec<String>>();
    let list_model = polaris::getIterator(track_data.clone()).await
        .map(|x| { x.0 }).collect::<Vec<String>>();
    let mut list_state = ListState::default().with_selected(Some(0));
    let mut index = 0;
    let mut toggle_play = false;
    loop {
        terminal.draw(|frame| render(frame, &mut list_state, &list_model))?;
//        if let Some(key) = event::read()?.as_key_press_event() {
//            match key.code {
//                KeyCode::Char('j') | KeyCode::Down => list_state.select_next(),
//                KeyCode::Char('k') | KeyCode::Up => list_state.select_previous(),
//                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
//                _ => {}
//            }
//        }

        match taskHandle
        {
            None   => { if toggle_play 
                      {  taskHandle = Some(task::spawn( listenerTask() ));
                         let mut list_polaris = polaris::getIterator(track_data.clone()).await;
                         index = list_state.selected().unwrap();
                         sendRequestToPlayer(list_polaris.nth(index).unwrap().1).await; 
                      } }

           Some(ref h) => { if h.is_finished() && toggle_play
                          { taskHandle = Some(task::spawn( listenerTask() ));
                            let mut list_polaris = polaris::getIterator(track_data.clone()).await;
                            index = index + 1;
                            list_state.select(Some(index));
                            sendRequestToPlayer(list_polaris.nth(index).unwrap().1).await;
                          }
                          else if h.is_finished() && !toggle_play
                          {
                            taskHandle = None;
                          } }
        }               

        if up.read() == 0.into() 
        { list_state.select_previous(); }
        
        if down.read() == 0.into()
        { list_state.select_next(); }

        if req.read() == 0.into()
        {  toggle_play = !toggle_play;
        }

        if quit.read() == 0.into()
        { 
          let _ = Command::new("sudo")
                      .arg("shutdown")
                      .arg("-h")
                      .arg("0")
                      .output()
                      .expect("Unable to shutdown.");
          // should not reach here
          return Ok(()); 
        }

        thread::sleep(Duration::from_millis(250));
    }
}

/// Render the UI with various lists.
fn render(frame: &mut Frame, list_state: &mut ListState, 
list_model: &Vec<String> ) {
    let vertical = Layout::default();
    //let [top] = vertical.areas(frame.area());

    //let title = Line::from_iter([
    //    Span::from("List Widget").bold(),
    //    Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    //]);
    //frame.render_widget(title.centered(), top);

    render_list(frame, frame.area(), list_state, list_model);
    //render_bottom_list(frame, second);
}

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

/// Render a list.
pub fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState,
list_model: &Vec<String> ) {
//    let items = ["Item 1: Long title ", "Item 2: Very long title", "Item 3: title", "Item 4",
//                 "Item 5: title",       "Item 6 : Very long title", "Item 7: Long title", 
//                 "Item 8: Incredibly long title", "Item 9: Another title", "Item 10: Guess what?",
//                 "Item 11: Incy wincy title", "Item 12: The last title?" ];
//    let items_v = vec!["Item 1", "Item 2", "Item 3\n+---> a)"];
//    let items_i = items_v.into_iter(); 
//    let items_c = items_i.map(|x| { Text::styled(x, Style::new().green()) } );
    let list = List::new(list_model.into_iter().map(|x| x.as_str()))
//        .style(Color::White)
        .highlight_style(SELECTED_STYLE);
//        .highlight_symbol("> ");

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
