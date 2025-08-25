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
use ratatui::widgets::{List, ListDirection, ListState, Paragraph};
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

use std::collections::HashSet;

mod polaris;

async fn getNextTrack(h: polaris::polarisHandle, s: &HashSet<usize>) -> String
{
    let mut list_polaris = polaris::getIterator(h).await;
    let index = s.iter().next().unwrap();
    list_polaris.nth(*index).unwrap().1 
}

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
    let mut terminal = ratatui::init();
    terminal.clear().unwrap();
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
    let left = gpio.get(27).unwrap().into_input();
    let right = gpio.get(23).unwrap().into_input();
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
    let mut playlist: HashSet<usize> = HashSet::new();
    loop {
        terminal.draw(|frame| render(frame, &mut list_state, &list_model, toggle_play, 
                                        &playlist))?;
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
                      {  if !playlist.is_empty()
                         {   taskHandle = Some(task::spawn( listenerTask() ));
                             sendRequestToPlayer(getNextTrack(track_data.clone(), 
                                 &playlist).await)
                             .await;
                          } 
                      } }

           Some(ref h) => { if h.is_finished() && toggle_play
                          { let curr_playlist = playlist.clone();
                            eprintln!("Curr Playlist len {}", curr_playlist.len());
                            let mut curr_iter = curr_playlist.iter();
                            eprintln!("Remaining iter length {}", curr_iter.len());
                            let index = curr_iter.next().unwrap();
                            playlist.remove(index);
                            if !playlist.is_empty()
                            {
                                taskHandle = Some(task::spawn( listenerTask() ));
                                sendRequestToPlayer(getNextTrack(track_data.clone(), 
                                    &playlist).await)
                                .await;
                            }
                            else
                            {
                                taskHandle = None;
                                toggle_play = false;
                            }
                          }
                          else if h.is_finished() && !toggle_play
                          {
                            let curr_playlist = playlist.clone();
                            let index = curr_playlist.iter().next().unwrap();
                            playlist.remove(index);
                            taskHandle = None;
                          } }
        }               

        if up.read() == 0.into() 
        { list_state.select_previous(); }
        
        if down.read() == 0.into()
        { list_state.select_next(); }

        if left.read() == 0.into()
        {  playlist.remove(&list_state.selected().unwrap()); }

        if right.read() == 0.into()
        {  playlist.insert(list_state.selected().unwrap()); }

        if req.read() == 0.into()
        {  toggle_play = !toggle_play; }

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
list_model: &Vec<String>, toggle_play: bool, l_playlist: &HashSet<usize> ) {
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
pub fn render_list(frame: &mut Frame, area: Rect, list_state: &mut ListState,
list_model: &Vec<String>, l_playlist: &HashSet<usize> ) {
    let list = List::new(list_model.into_iter().map(|x| x.as_str())
        .enumerate()
        .map(|(i,x)| { if l_playlist.contains(&i)
                       { Span::styled(x, Style::default().fg(Color::Yellow)) }
                       else
                       { Span::styled(x, Style::default().fg(Color::White))  } 
                     } ) )
//    .highlight_style(SELECTED_STYLE);
    .highlight_style(Modifier::UNDERLINED);
    frame.render_stateful_widget(list, area, list_state);
}

/// Render a bottom-to-top list.
pub fn render_bottom(frame: &mut Frame, area: Rect, auto_play: bool) {
    let final_text;
    match auto_play
    {
        false => final_text = String::from("\n             "),
        true  => {    let mut temp_text = String::from("\n             ");
                      temp_text.extend(["A"]);
                      final_text = temp_text; }
    }
    let text = Paragraph::new(final_text);
    frame.render_widget(text, area);
}
