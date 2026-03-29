// use tokio::fs::File;
use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source, Sink};
// use tokio::net::TcpStream;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::options;
use crate as player;
use std::sync::{Arc,Mutex};
// use tokio::time::{Duration, sleep};
// use tokio::io::{AsyncWriteExt};
use std::time::{Duration};
use std::path::Path;

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
	let mut stream = TcpStream::connect(&tui_address).unwrap();
	let mut temp_path = String::from("music/");
    temp_path.extend([&path,""]);
	println!("Temp path: {}", temp_path);
	let path_s = Path::new(&temp_path);
	if path_s.try_exists().unwrap()
	{
		let file = File::open(path_s).unwrap();
		// Decode that sound file into a source
		let source = Decoder::try_from(file).unwrap();
		sink.append(source);
		// while !sink.empty() {
		// 	sleep(Duration::from_secs(1));
		// }
		sink.sleep_until_end();
	}
	else {
		std::fs::create_dir_all(path_s.parent().unwrap()).unwrap();
		let response = player::getResponse(String::from(path));
		let stream = player::StreamingAdapter::from_stream(response, path_s.to_str().unwrap());
		let decoder = Decoder::new(stream).unwrap();
		// let handle = rodio::OutputStreamBuilder::open_default_stream()
		// 		.expect("open default audio stream");
		// let sink = rodio::Sink::connect_new(&handle.mixer());
		sink.append(decoder);
		sink.sleep_until_end();
	}

	stream.write(&[1]);
}	
