//! Demonstrates construction and usage of a TLS-capable HTTP client.

use tls_rustls_0_23 as rustls;

use std::{error::Error as StdError, sync::Arc};
use std::{fs, io::Read};

use reqwest::Client;
use reqwest::Url;
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::CertificateDer;
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::task::spawn_blocking;
use webpki_roots::TLS_SERVER_ROOTS;

mod auth;

pub type polarisHandle = serde_json::Value;

pub async fn getBody() -> Result<serde_json::Value, ()> {
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
    //println!("Certificate 2 bytes read: {}", cert_byte_count2);

    // Read certificate 3
    let mut cert_file3 = tokio::fs::File::open("4256644734.der")
        .await
        .expect("Failed to open cert 3 file");
    let mut data_buf3 = Vec::new();
    let cert_byte_count3 = cert_file3
        .read_to_end(&mut data_buf3)
        .await
        .expect("Failed to read cert 3 file");
    //println!("Certificate 3 bytes read: {}", cert_byte_count3);

    //rustls::crypto::aws_lc_rs::default_provider().install_default();

    // Build reqwest client
    let client = Client::builder()
        .use_rustls_tls()
        .add_root_certificate(
            reqwest::tls::Certificate::from_der(&data_buf2).unwrap(),
        )
        .add_root_certificate(
            reqwest::tls::Certificate::from_der(&data_buf3).unwrap(),
        )
        .build()
        .expect("Failed to build reqwest client");

    // Target URL
    //let mut base_url = String::from("https://www.emstreamer.online/api/audio/");
    //base_url.extend([path.as_str()]);
    //println!("Url : {}", &base_url);
    let url = Url::parse(
        "https://www.emstreamer.online/api/flatten/", //        &base_url
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
    //println!("Status: {}", response.status());

    // Read the body
    let body = response
        .json::<serde_json::Value>()
        .await
        .expect("Error downloading.");
    //println!("Downloaded: {}", body.len());
    Ok::<_, _>(body)
}

pub async fn getData(
    data: serde_json::Value,
) -> impl Iterator<Item = (String, String, String)> {
    if let Value::Array(vec) = data {
        let tracks = vec.into_iter();
        let id_iter = tracks.map(|x| {
            if x.is_object() {
                (
                    String::from(x["path"].as_str().unwrap()),
                    String::from(x["artist"].as_str().unwrap()),
                    String::from(x["title"].as_str().unwrap()),
                )
            } else {
                (String::from("?"), String::from("?"), String::from("?"))
            }
        });
        return id_iter;
    } else {
        panic!("Value not an array.");
    }
}

pub async fn polaris() -> impl Iterator<Item = (String, String)> {
    let mut data = getData(getBody().await.unwrap()).await;
    let result = data.into_iter().map(|x| {
        let mut s = String::new();
        s.extend([
            "* ".to_string(),
            x.1,
            " /".to_string(),
            "\n  ".to_string(),
            x.2,
        ]);
        (s, x.0)
    });
    result
}

pub async fn getIterator(
    h: polarisHandle,
) -> impl Iterator<Item = (String, String)> {
    let mut data = getData(h).await;
    let result = data.into_iter().map(|x| {
        let mut s = String::new();
        s.extend([
            "* ".to_string(),
            x.1,
            " /".to_string(),
            "\n  ".to_string(),
            x.2,
        ]);
        (s, x.0)
    });
    result
}
