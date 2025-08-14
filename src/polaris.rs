//! Demonstrates construction and usage of a TLS-capable HTTP client.

use tls_rustls_0_23 as rustls;

use std::{error::Error as StdError, sync::Arc};
use std::{fs, io::Read};

use actix_tls::connect::rustls_0_23::webpki_roots_cert_store;
use rustls::{ClientConfig, RootCertStore};
use rustls_pki_types::CertificateDer;

use awc::JsonBody;
use awc::http::Uri;
use serde_json::Value;

mod auth;

pub async fn getBody() -> Result<serde_json::Value, ()> {
    //env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let mut cert_file2 = fs::File::open(r"4267304690.der").unwrap();
    let mut data_buf2 = Vec::<u8>::new();
    let cert_byte_count2 = cert_file2.read_to_end(&mut data_buf2).unwrap();
    //println!("Certificate 2 bytes read :{}", cert_byte_count2);

    let mut cert_file3 = fs::File::open(r"4256644734.der").unwrap();
    let mut data_buf3 = Vec::<u8>::new();
    let cert_byte_count3 = cert_file3.read_to_end(&mut data_buf3).unwrap();
    //println!("Certificate 3 bytes read :{}", cert_byte_count3);

    let mut root_store = webpki_roots_cert_store();
    root_store
        .add(CertificateDer::from_slice(data_buf2.as_slice()))
        .expect("Adding cert 2 failed.");
    root_store
        .add(CertificateDer::from_slice(data_buf3.as_slice()))
        .expect("Adding cert 3 failed.");

    let mut config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let protos = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    config.alpn_protocols = protos;

    // construct request builder with TLS support
    let client = awc::Client::builder()
        .connector(awc::Connector::new().rustls_0_23(Arc::new(config)))
        .finish();
    /*
        let uri = Uri::builder()
            .scheme("https")
            .authority("www.emstreamer.online")
            .path_and_query("/api/audio/AwsMusic/Music/Cannons/Desire%020-%020Single/01%020Desire.m4a")
            .build()
            .unwrap();
    */
    // configure request
    let request = client
        //	    .get(uri)
        .get("https://www.emstreamer.online/api/flatten")
        .bearer_auth(auth::token);

    //println!("Request: {request:?}");

    let mut response = request.send().await.unwrap();

    // server response head
    //println!("Response: {response:?}");

    // read response body
    let body = response
        .json::<serde_json::Value>()
        .limit(7000000)
        .await
        .unwrap();
    //println!("Downloaded: {}", body.to_string());

    //    assert!(body.is_object());

    Ok::<_, _>(body)
}

pub fn getData(data: serde_json::Value) -> impl Iterator<Item = (String, String, String)> {
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


#[actix_rt::main]
pub async fn polaris() -> impl Iterator<Item=(String,String)> {
    let mut data = getData(getBody().await.unwrap());
    let result = data.into_iter().map(|x| 
        { let mut s = String::new(); s.extend([
              "* ".to_string(),x.1," /".to_string(),"\n  ".to_string(),x.2]);
          (s, x.0) } );
    result
}

