use crate::tui;
use crate::tui::screen1::{model::Model, view::View, ModelCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::polaris::{self, polarisHandle};
use crate::tui::screen1::{State, Output};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Clone)]
pub struct Controller {
    pub data: polarisHandle,
}

pub struct ControllerState {
    pub s: i32,
	pub b: i8,
	pub rx: Receiver<Option<()>>
}


impl<'c> Compute<'c> for Controller {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        _: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 6],
    ) -> Output {
        Output::Model(Model { data : self.data,
			cmd : ModelCommand::Init	})
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
		    data : polaris::getBody().await.unwrap(),
		}
		
	}
	
}