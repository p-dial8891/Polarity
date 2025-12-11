use tokio::fs::File;
use rodio::{Decoder, OutputStream, source::Source, Sink};
use tokio::net::TcpStream;
use std::io::prelude::*;
use crate::options;
use std::sync::{Arc,Mutex};
use tokio::time::{Duration, sleep};
use tokio::io::{AsyncWriteExt};

pub async fn play(path: &str, sink: Arc<Sink>)
{
	let mut tui_address = options::getTuiAddress();
	tui_address.extend([":9000"]);
	let mut stream = TcpStream::connect(&tui_address).await.unwrap();

	let file = File::open(path).await.unwrap();
	// Decode that sound file into a source
	let source = Decoder::try_from(file.into_std().await).unwrap();
	// Play the sound directly on the device
	//stream_handle.mixer().add(source);
	
	sink.append(source);

    while !sink.empty() {
	    sleep(Duration::from_secs(1)).await;
	}

	stream.write(&[1]).await;
}	
