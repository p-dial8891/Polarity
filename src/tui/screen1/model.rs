use crate::tui;
use crate::tui::screen1::{controller::Controller, view::View, 
    ModelCommand::{self, Init} };
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::tui::screen1::{State, Output};
use crate::polaris::{self, polarisHandle};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Clone)]
pub struct Model {
    pub data: polarisHandle,
	pub cmd: ModelCommand
}

pub struct ModelState {
    pub s: u32,
    pub b: u16,
	pub list: Rc<Vec<String>>,
	pub tx : Sender<Option<()>>
}

impl<'c> Compute<'c> for Model {
    type State = State;
    type Output = Output;

    async fn compute(
	    self, 
		s: &mut State, 
		_: &mut DefaultTerminal, 
		_: [&'c InputPin; 6],
	) -> Output {
		
		match self.cmd {
			
			Init => { 
			    s.unwrap_model().list = Rc::new(polaris::getIterator(self.data.clone())
                    .await
                    .map(|x| x.0)
                    .collect::<Vec<String>>());
		    },
			_ => {()}
		}
			
        Output::View(View { data : self.data,
            list : s.unwrap_model().list.clone()	})
    }
}
