use ratatui::{
    DefaultTerminal,
	Terminal as ratTerminal,
	backend::CrosstermBackend,
    CompletedFrame,
    backend::Backend,
    prelude::Size
};
use std::io::{Stdout, Write, IsTerminal};
use std::fs::{File};
use tokio::fs::{
    // File as tokFile,
    OpenOptions
};
use crate::tui::output::Terminal::*;
use crate::options;

pub async fn getDisplayFd() -> std::fs::File {
	let mut tty_async = OpenOptions::new()
        .read(true)
        .write(true)
        .open(options::getDisplay().as_str())
        .await
        .unwrap();
	let mut tty = tty_async.try_into_std().unwrap();
    if !tty.is_terminal() {
        panic!("<APP> : Opened a non tty.")
    }
	tty
}

pub enum Terminal {
    Console(DefaultTerminal),
    Display(ratTerminal<CrosstermBackend<File>>)
}

impl Terminal {
    pub fn new_console() -> Self {
        Console(ratatui::init())
    }

    pub async fn new_display() -> Self {
        let mut backend = CrosstermBackend::new(getDisplayFd().await);
        let mut t_display = ratTerminal::new(backend).expect("Could not create display terminal.");       
        Display(t_display)
    }

    pub fn draw<F: for<'a, 'b> FnOnce(&'a mut ratatui::Frame<'b>)>(
        &mut self,
        render_callback: F,
    ) -> Result<CompletedFrame<'_>, std::io::Error> {
        match self {
            Console(t) => {
                t.draw(render_callback).map_err(|_| { std::io::Error::last_os_error() })
            }
            Display(t) => {
                t.draw(render_callback).map_err(|_| { std::io::Error::last_os_error() })
            }
        }
    }

    pub fn size(&self) -> Result<Size, std::io::Error> {
        match self {
            Console(t) => {
                t.size().map_err(|_| { std::io::Error::last_os_error() })
            }
            Display(t) => {
                t.size().map_err(|_| { std::io::Error::last_os_error() })
            }
        }
    }

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        match self {
            Console(t) => {
                t.clear().map_err(|_| { std::io::Error::last_os_error() })
            }
            Display(t) => {
                t.clear().map_err(|_| { std::io::Error::last_os_error() })
            }
        }
    }
}