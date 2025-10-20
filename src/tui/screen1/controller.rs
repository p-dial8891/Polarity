use crate::tui;
use crate::tui::screen1::{model::Model, view::View, ModelCommand};
use crate::tui::{Components, Compute, IntoComponent, IntoComp};
use ratatui::DefaultTerminal;
use rppal::gpio::{self, InputPin};
use crate::polaris::{self, polarisHandle};
use crate::tui::screen1::{State, Output};
use std::rc::Rc;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::tui::app::{
    UP_KEY, DOWN_KEY, LEFT_KEY, RIGHT_KEY, QUIT_KEY, REQ_KEY };
use std::process::Command;
use tokio::task;

#[derive(Clone)]
pub struct Controller {
    pub data: polarisHandle,
}

pub struct ControllerState {
    pub start: bool,
	pub task: Option<task::JoinHandle<()>>,
	pub rx: Receiver<Option<task::JoinHandle<()>>>
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
		let state_data = s.unwrap_controller();
		
		if state_data.start == true {
            state_data.start = false;
			eprintln!("<Controller> : Initialised.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::Init	});
		}
		
		if let Some(t) = &state_data.task {
			if t.is_finished() {
				state_data.task = None;
				return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::PlaybackFinished	});
			}
		}
		
		match state_data.rx.try_recv() {
			Ok(t_handle) => { state_data.task = t_handle; }
			_ => {}
		}
	
		if gpio_pins[UP_KEY].read() == 0.into() {
			eprintln!("<Controller> : Up key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::SelectPrevious	});
		}
		if gpio_pins[DOWN_KEY].read() == 0.into() {
			eprintln!("<Controller> : Down key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::SelectNext	});
		}
		if gpio_pins[LEFT_KEY].read() == 0.into() {
			eprintln!("<Controller> : Left key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::RemoveTrack	});
		}
		if gpio_pins[RIGHT_KEY].read() == 0.into() {
			eprintln!("<Controller> : Right key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::AddTrack	});
		}
		if gpio_pins[REQ_KEY].read() == 0.into() {
			eprintln!("<Controller> : Request key pressed.");
			return Output::Model(Model { data : self.data,
			    cmd : ModelCommand::TogglePlay	});
		}
		if gpio_pins[QUIT_KEY].read() == 0.into() {
			eprintln!("<Controller> : Quit key pressed.");
            let _ = Command::new("sudo")
                .arg("shutdown")
                .arg("-h")
                .arg("0")
                .output()
                .expect("Unable to shutdown.");
		}
		// should not matter what happens from here.	
        Output::Model(Model { data : self.data,
			cmd : ModelCommand::Noop	})
    }
}

impl Controller {
	
	pub async fn new() -> Self {
		
		Controller {
		    data : polaris::getBody().await.unwrap(),
		}
		
	}
	
}