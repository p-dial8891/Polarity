use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};
use std::net::TcpStream;
use std::io::prelude::*;

pub fn play(path: &str)
{
	let mut stream = TcpStream::connect("raspberrypi.local:9000").unwrap();

	// Get an output stream handle to the default physical sound device.
	// Note that the playback stops when the stream_handle is dropped.//!
	let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
			.expect("open default audio stream");
	let sink = rodio::Sink::connect_new(&stream_handle.mixer());
	// Load a sound from a file, using a path relative to Cargo.toml
	let file = File::open(path).unwrap();
	// Decode that sound file into a source
	let source = Decoder::try_from(file).unwrap();
	// Play the sound directly on the device
	//stream_handle.mixer().add(source);
	sink.append(source);

	sink.sleep_until_end();
	//std::thread::sleep(std::time::Duration::from_secs(60*5));

	stream.write(&[1]).unwrap();
}	
