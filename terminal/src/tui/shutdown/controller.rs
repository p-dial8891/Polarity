use crate::tui;
use crate::tui::shutdown::{model::Model, view::View, ModelCommand, ControllerCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::tui::shutdown::{State, Output};
use crate::tui::app::{
    UP_KEY, DOWN_KEY, LEFT_KEY, RIGHT_KEY, REQ_KEY };
use std::process::Command;

#[derive(Clone)]
pub struct Controller {
    pub cmd : ControllerCommand
}

pub struct ControllerState {
    pub start : bool
}


impl<'c> Compute<'c> for Controller {
    type State = State;
    type Output = Output;

    async fn compute(
        self,
        s: &mut State,
        _: &mut DefaultTerminal,
        gpio_pins: [&'c InputPin; 5],
    ) -> Output {
		
		let state_data = s.unwrap_controller();
		
		match self.cmd {
			ControllerCommand::Init => {
				state_data.start = false;
				eprintln!("<Controller> : Initialised.");
				return Output::Model(Model { cmd : ModelCommand::Init	});
			},
			_ => {}
		}
		
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { cmd : ModelCommand::Init	});
		}
		
 		if gpio_pins[REQ_KEY].read() == 0.into() {
			eprintln!("<Controller> : Quit key pressed.");
            let _ = Command::new("sudo")
                .arg("shutdown")
                .arg("-h")
                .arg("0")
                .output()
                .expect("Unable to shutdown.");
			return Output::Model(Model { cmd : ModelCommand::Noop	});
		}
		
		Output::Model(Model { cmd : ModelCommand::Noop	})
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
            cmd: ControllerCommand::Init
		}
		
	}
	
}