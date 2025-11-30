use rppal::gpio::{self, Gpio, InputPin, Level};
use crossterm::{
    event::{poll, read, Event, KeyCode}
};

pub struct InputConfig {
	pin : InputPin,
	ch : char,
}

impl InputConfig {
	
	pub fn init(gpio : &Gpio, pin : u8, c : char) -> InputConfig {
		
		InputConfig {
			pin : gpio.get(pin).unwrap().into_input(),
			ch : c,
		}

	}
	
	pub fn read(&self, e : &mut Option<Event>) -> Level {
		
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
	
	pub keys : [InputConfig; 5],
	pub ev : Option<Event>
	
}

impl Input {
	
	pub fn init( k : [InputConfig;5] ) -> Input {
		
		Input {
			keys : k,
			ev : None
		}
	}
	
    pub fn set_event(&mut self, c : Event ) {
		
		self.ev = Some(c);
		
	}
}
