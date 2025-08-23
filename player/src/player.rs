
use clap::Parser;
use futures::{future, prelude::*};
use tarpc::{
    context,
    server::{self, Channel, incoming::Incoming},
    tokio_serde::formats::Json,
};

use service::{Player, init_tracing};
use std::io::Error;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use tokio::fs;
use tokio::fs::{File, write};
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;
use tokio::runtime::Runtime;

use std::error::Error as OtherError;
use std::io::BufReader;
use std::pin::Pin;
use std::path::Path;
use tls_rustls_0_23 as rustls;

use std::sync::Arc;
use reqwest::Client;
use reqwest::Url;
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::CertificateDer;
use webpki_roots::TLS_SERVER_ROOTS;
use tokio::task::spawn_blocking;
 
use bytes::Bytes;
 
mod auth;
mod audio;

/*
#[derive(Parser)]
struct Flags {
    /// Sets the secret key to use
    #[clap(long)]
    secret: String,
    /// Sets the port number to listen on.
    #[clap(long)]
    port: u16,
}
*/
// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
struct PlayerServer(SocketAddr);

async fn getBody(path: String) -> Bytes {
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Read certificate 2
    let mut cert_file2 = tokio::fs::File::open("4267304690.der")
        .await
        .expect("Failed to open cert 2 file");
    let mut data_buf2 = Vec::new();
    let cert_byte_count2 = cert_file2
        .read_to_end(&mut data_buf2)
        .await
        .expect("Failed to read cert 2 file");
    println!("Certificate 2 bytes read: {}", cert_byte_count2);

    // Read certificate 3
    let mut cert_file3 = tokio::fs::File::open("4256644734.der")
        .await
        .expect("Failed to open cert 3 file");
    let mut data_buf3 = Vec::new();
    let cert_byte_count3 = cert_file3
        .read_to_end(&mut data_buf3)
        .await
        .expect("Failed to read cert 3 file");
    println!("Certificate 3 bytes read: {}", cert_byte_count3);

    rustls::crypto::aws_lc_rs::default_provider().install_default();

    // Build reqwest client
    let client = Client::builder()
		.use_rustls_tls()
        .add_root_certificate(reqwest::tls::Certificate::from_der(&data_buf2).unwrap())
        .add_root_certificate(reqwest::tls::Certificate::from_der(&data_buf3).unwrap())
        .build()
        .expect("Failed to build reqwest client");

    // Target URL
	let mut base_url = String::from("https://www.emstreamer.online/api/audio/");
	base_url.extend([path.as_str()]);
	println!("Url : {}", &base_url);
    let url = Url::parse(
//        "https://www.emstreamer.online/api/audio/AwsMusic/Music/Cannons/Desire - Single/01 Desire.m4a"
        &base_url
    )
    .expect("Invalid URL");

    // Send request with Bearer token
    let mut response = client
        .get(url)
        .bearer_auth(auth::token) // your token variable here
        .send()
        .await
        .expect("HTTP request failed");

    // Read the body
    println!("Status: {}", response.status());

    // Read the body
    let body = response.bytes().await.expect("Error downloading.");
    println!("Downloaded: {}", body.len());
	
/*
    // Create output file for streamed download
    let mut file = tokio::fs::File::create("downloaded.m4a")
        .await
        .expect("Failed to create output file");

    let mut downloaded: u64 = 0;
    while let Some(chunk) = response.chunk().await.expect("Error reading chunk") {
        file.write_all(&chunk)
            .await
            .expect("Failed to write chunk to file");
        downloaded += chunk.len() as u64;
        println!("Downloaded {} bytes so far...", downloaded);
    }

    println!("Download complete. Total size: {} bytes", downloaded);
*/
    body
}

impl Player for PlayerServer {
    async fn play(self, _: context::Context, path: String) -> Result<(),()> {
		println!("Path recvd: {}", path);
		let root_path = Path::new("music");
		let temp_path = root_path.join(&path);
		let final_path = &temp_path;
		println!("Final path: {}", final_path.display());
		if !final_path.try_exists().unwrap()
		{
    	  let data = getBody(path).await;
		  tokio::fs::create_dir_all(final_path.parent().unwrap()).await.unwrap();
		  write(final_path, data).await.unwrap();
		}
		tokio::task::spawn_blocking(move || {
		    audio::play(temp_path.as_path().to_str().unwrap()); 
			} );
		Ok(())
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = (IpAddr::V4(Ipv4Addr::new(169, 254, 108, 7)), 50051);

    // JSON transport is provided by the json_transport tarpc module. 
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    tracing::info!("Listening on port {}", listener.local_addr().port());
    println!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| {
            let server = PlayerServer(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve()).for_each(spawn)
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
/*
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Tarpc Example Server")?;
    println!("Tarpc Example Server");

    // Open the file in read-only mode with buffer.
    let contents = fs::read_to_string("input.json")
        .await
        .expect("Unable to open JSON file.");
    //let contents = decrypt(&encrypted, flags.secret.as_bytes())
    //.expect("Failed to decrypt the file");
    // Read the JSON contents of the file as an instance of `User`.
    let server_addr = (IpAddr::V4(Ipv4Addr::new(169, 254, 24, 24)), flags.port);

    // JSON transport is provided by the json_transport tarpc module. It makes it easy
    // to start up a serde-powered json serialization strategy over TCP.
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    tracing::info!("Listening on port {}", listener.local_addr().port());
    println!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        // Ignore accept errors.
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        // Limit channels to 1 per IP.
        .max_channels_per_key(1, |t| t.transport().peer_addr().unwrap().ip())
        // serve is generated by the service attribute. It takes as input any type implementing
        // the generated World trait.
        .map(|channel| {
            let v: Value = serde_json::from_str(contents.as_str()).expect("Unable to parse JSON");
            let server = MobiusServer(
                channel.transport().peer_addr().unwrap(),
                v.clone(),
                flags.secret.clone(),
            );
            channel.execute(server.serve()).for_each(spawn)
            //fs::remove_file("input.json")
        })
        // Max 10 channels.
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
*/