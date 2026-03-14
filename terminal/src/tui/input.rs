use crossterm::{
    event::{poll, read, Event, KeyCode, EventStream}
};
use crate::tui::app::Keys::{self, *};
use futures::{StreamExt, FutureExt, select};
use tokio::{time};
use std::time::Duration;
use std::convert::Infallible;
use std::io::ErrorKind::InvalidInput;

pub struct InputConfig {
	pin : u8,
	ch : KeyCode,
}

impl InputConfig {
	
	pub fn init(pin : u8, c : KeyCode) -> InputConfig {
		
		InputConfig {
			pin : pin,
			ch : c,
		}

	}
	
	fn read(&mut self, e : &mut Option<Event>) -> bool {
		match e {
			Some(ev) => {
				if *ev == Event::Key(self.ch.into())   {
					eprintln!("Key {:?} captured", self.ch);
					*e = None;
					false
				} else {
					true
				}
			}
			None => true
		}
	}

	fn peek(&self, e : &Option<Event>) -> bool {
		match e {
			Some(ev) => {
				if *ev == Event::Key(self.ch.into())   {
					eprintln!("Key {:?} captured", self.ch);
					false
				} else {
					true
				}
			}
			None => true
		}
	}
}

#[cfg(feature = "enable-rppal")]
use rppal::gpio::{self, Gpio, InputPin, Level};

#[cfg(feature = "enable-rppal")]
pub struct InputInitialised {
	pub pin : InputPin,
	pub ch : KeyCode
}

#[cfg(feature = "enable-rppal")]
impl InputInitialised {
	
	fn read(&mut self, e : &mut Option<Event>) -> Level {
		
		match self.pin.read() {
			
			Level::Low => Level::Low,
			Level::High => {
				match e {
					Some(ev) => {
						if *ev == Event::Key(self.ch.into())  {
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

#[cfg(feature = "enable-rppal")]
pub struct Input {
	
	pub keys : [InputInitialised; 7],
	pub ev : Option<Event>
	
}

#[cfg(feature = "enable-rppal")]
impl Input {
	
	pub fn init( k : [InputConfig;7] ) -> Input {
		
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
				    ch: k[TAB_KEY as usize].ch },
				  InputInitialised { 
				    pin: gpio.get(k[FIND_KEY as usize].pin).unwrap().into_input(), 
				    ch: k[FIND_KEY as usize].ch } ]
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

#[cfg(not(feature = "enable-rppal"))]
pub struct Input {
	
	pub keys : [InputConfig; 7],
	pub ev : Option<Event>,
	pub reader : EventStream,

}

#[cfg(not(feature = "enable-rppal"))]
impl Input {
	
	pub fn init( k : [InputConfig;7] ) -> Input {
		
		Input {
			keys : k,
			ev : None,
			reader : EventStream::new()
		}
	}
	
    pub fn set_event(&mut self, c : Event ) {
		
		self.ev = Some(c);
		
	}
	
	pub fn read(&mut self, k : Keys) -> bool {
		
		self.keys[k as usize].read(&mut self.ev)
	}

	pub fn peek(&self, k : Keys) -> bool {
		
		self.keys[k as usize].peek(&self.ev)
	}

	pub async fn read_edit(&mut self, buf: &mut [u8]) -> Result<i16, std::io::ErrorKind> {
		let mut used = 0;
		loop {
			let mut event = self.reader.next().fuse();
			select! {
				ev = event => {
					match ev {
						Some(Ok(Event::Key(k))) => {
							if !k.is_press() {
								continue; 
							}
							match k.code {
								KeyCode::Tab | KeyCode::Esc => {
									self.set_event(Event::Key(k));
									break;
								}
								KeyCode::Char(c) => {
									self.set_event(Event::Key(k));
									buf[<i16 as TryInto<usize>>::try_into(used).unwrap()] = c as u8;
									used += 1;
									break;
								}
								KeyCode::Backspace => {
									self.set_event(Event::Key(k));
									used -= 1;
									break;
								}
								_ => { return Err(InvalidInput.into()) } 
							}
						},
						None => { continue; }
						_ => { return Err(InvalidInput.into()) } 
					}
				},
				_ = async {
					tokio::time::sleep(Duration::from_millis(5)).await;
				}.fuse() => { break; }
			}
		}
		Ok(used)
	}
}

