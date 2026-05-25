// use tokio::fs::File;
// use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source, Sink};
// use tokio::net::TcpStream;
// use std::net::TcpStream;
// use std::io::prelude::*;
use crate::options;
use crate as player;
use std::sync::{Arc,Mutex};
// use tokio::time::{Duration, sleep};
// use tokio::io::{AsyncWriteExt};
// use std::time::{Duration};
// use std::path::Path;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpStream, Shutdown};
use std::thread::{sleep};
use std::time::{Duration, Instant};
use std::io::Write;

// pub async fn play(path: &str, sink: Arc<Sink>)
// {
// 	let mut tui_address = options::getTuiAddress();
// 	tui_address.extend([":9000"]);
// 	let mut stream = TcpStream::connect(&tui_address).await.unwrap();
// 	if path.try_exists().unwrap()
// 	{
// 		let file = File::open(path).await.unwrap();
// 		// Decode that sound file into a source
// 		let source = Decoder::try_from(file.into_std().await).unwrap();
// 		sink.append(source);
// 		while !sink.empty() {
// 			sleep(Duration::from_secs(1)).await;
// 		}		
// 	}
// 	else {
// 		std::fs::create_dir_all(path.parent().unwrap()).unwrap();
// 		let response = player::getResponse(String::from(path));
// 		let stream = player::StreamingAdapter::from(response, path);
// 		let decoder = Decoder::new(stream).unwrap();
// 		// let handle = rodio::OutputStreamBuilder::open_default_stream()
// 		// 		.expect("open default audio stream");
// 		// let sink = rodio::Sink::connect_new(&handle.mixer());
// 		sink.append(decoder);
// 		sink.sleep_until_end();
// 	}

// 	stream.write(&[1]).await;
// }	

pub fn play(path: &str, sink: Arc<Sink>)
{
	let mut tui_address = options::getTuiAddress();
	tui_address.extend([":9000"]);
	let mut stream_tui = TcpStream::connect(&tui_address).unwrap();
	let mut temp_path = String::from("music/");
    temp_path.extend([&path,""]);
	println!("Temp path: {}", temp_path);
	let path_s = Path::new(&temp_path);
	if path_s.try_exists().unwrap()
	{
		let mut input = File::open(path_s).unwrap();
		let mut buffer = [0u8; 90112/(2)];
		let mut ready_buffer = [0];

		let mut retry_count = 8;
		let mut profile_count = 4;
		let mut offset = 0;
		let mut written = 0;
		let mut written_sum = 0;
		let mut profile_duration = 0;
		let mut profile_start = Instant::now();
		let mut read = input.read(&mut buffer).unwrap();

		let address = options::getMicroPolarityAddress();

		while retry_count > 0 {
			match TcpStream::connect(address.as_str()) {
				Ok(mut stream) => {
					println!("TCP Stream read timmeout : {:?} and write timeout : {:?}", 
						stream.read_timeout(), stream.write_timeout());

					while read > 0 {
						written = match stream.write(&buffer[..(read + offset)]) {
							Ok(n) => {n},
							Err(e) => {
								println!("Error writing to mp device. Terminating.");
								return
							}
						};
						written_sum += written;
						//sleep(Duration::from_millis(5632/(2*2*2*2)));
						buffer.copy_within(written..read,0);
						offset = read - written;
						if profile_count == 0 {
							profile_duration = Instant::now().duration_since(profile_start).as_millis();
							println!("Read : {} , Written : {} , Offset : {} @ {:.3}Kbps", read, written, offset,
								(written_sum as f32)*2f32/(profile_duration as f32));
							profile_count = 4;
							written_sum = 0;
							profile_duration = Duration::ZERO.as_millis();
							profile_start = Instant::now();
						}
						read = input.read(&mut buffer[offset..]).unwrap();
						profile_count -= 1;
					}
				
					println!("Shutting down.");
					stream.shutdown(Shutdown::Both).expect("Shutdown failed.");
					// println!("Waiting for connection reset.");
					// while let Ok(_) = stream.read(&mut ready_buffer) {
					// 	sleep(Duration::from_millis(500));
					// }

					break;
				},

				Err(_) => {
					sleep(Duration::from_millis(500));
					retry_count -= 1;
					continue;
				}
			}
		}
	}
	// else {
	// 	std::fs::create_dir_all(path_s.parent().unwrap()).unwrap();
	// 	let response = player::getResponse(String::from(path));
	// 	let stream = player::StreamingAdapter::from_stream(response, path_s.to_str().unwrap());
	// 	let decoder = Decoder::new(stream).unwrap();
	// 	// let handle = rodio::OutputStreamBuilder::open_default_stream()
	// 	// 		.expect("open default audio stream");
	// 	// let sink = rodio::Sink::connect_new(&handle.mixer());
	// 	sink.append(decoder);
	// 	sink.sleep_until_end();
	// }

	stream_tui.write(&[1]);
}	
