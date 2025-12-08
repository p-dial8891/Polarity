use rppal::gpio::{self, Gpio, InputPin, Level};
use crossterm::{
    event::{poll, read, Event, KeyCode}
};
use crate::tui::app::Keys::{self, *};

pub struct InputConfig {
	pin : u8,
	ch : char,
}

impl InputConfig {
	
	pub fn init(pin : u8, c : char) -> InputConfig {
		
		InputConfig {
			pin : pin,
			ch : c,
		}

	}
	
}

pub struct InputInitialised {
	pub pin : InputPin,
	pub ch : char
}

impl InputInitialised {
	
	fn read(&mut self, e : &mut Option<Event>) -> Level {
		
		match self.pin.read() {
			
			Level::Low => Level::Low,
			Level::High => {
				match e {
					Some(ev) => {
						if *ev == Event::Key(KeyCode::Char(self.ch).into()) ||
                           ( *ev == Event::Key(KeyCode::Enter.into()) && 
						       self.ch == '\u{000A}' ) ||
                           ( *ev == Event::Key(KeyCode::Tab.into()) &&
                               self.ch == '\u{0009}' )   {
							eprintln!("Key {:?} captured", self.ch);
							*e = None;
							Level::Low
						} else {
							Level::High
						}
					}
                    None => Level::High					
				}
	    	}
	    }
	}
}	

pub struct Input {
	
	pub keys : [InputInitialised; 6],
	pub ev : Option<Event>
	
}

impl Input {
	
	pub fn init( k : [InputConfig;6] ) -> Input {
		
		Input {
			keys : {
				let gpio = Gpio::new().unwrap();
				[ InputInitialised { 
				    pin: gpio.get(k[UP_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[UP_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[DOWN_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[DOWN_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[LEFT_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[LEFT_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[RIGHT_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[RIGHT_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[REQ_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[REQ_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[TAB_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[TAB_KEY as usize].ch } ]
			},
			ev : None
		}
	}
	
    pub fn set_event(&mut self, c : Event ) {
		
		self.ev = Some(c);
		
	}
	
	pub fn read(&mut self, k : Keys) -> bool {
		
		match self.keys[k as usize].read(&mut self.ev) {
			
			Level::Low => false,
			Level::High => true
		}
	
	}

}
