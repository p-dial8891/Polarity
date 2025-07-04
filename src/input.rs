use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() -> Result<(), Box<dyn std::error::Error>>
{

  let gpio = Gpio::new()?;
  let up = gpio.get(17)?.into_input();
  let down = gpio.get(22)?.into_input(); 

  loop {
    if up.read() == 0.into()
    {
      println!("\rPin 17 is down");
    }
    if down.read() == 0.into()
    {
      println!("\rPin 22 is down");
    } 
    thread::sleep(Duration::from_secs(1));
  }
  
}
