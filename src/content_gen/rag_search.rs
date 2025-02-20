#[cfg(feature = "sync")]
use native_tls::TlsConnector;
#[cfg(feature = "sync")]
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

#[cfg(feature = "async")]
use async_std::net::TcpStream;

#[cfg(feature = "async")]
use async_tls::TlsConnector;

use std::panic;

#[cfg(feature = "sync")]
pub fn search(source: &str) -> String {
    let domain = domain(source);
    let stream = format!("{}:443", domain);
    let tls = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(stream).unwrap();
    let mut client = tls.connect(&domain, stream).unwrap();

    let get = format!("GET {} HTTP/1.1\r\nConnection: close\r\n\r\n", source);
    client.write_all(get.as_bytes());
    client.flush();

    let mut b = String::new();
    let reader = BufReader::new(client);
    for line in reader.lines() {
        let billionaire = line.unwrap();
        b.push_str(&billionaire);
    }
    // println!("{}", source);
    b
}

#[cfg(feature = "async")]
pub async fn search(source: &str) -> String {
    use async_std::io::{BufReadExt, BufReader, ReadExt, WriteExt};

    let domain = domain(source);
    let stream = format!("{}:443", domain);
    let tls = TlsConnector::new();
    let stream = TcpStream::connect(stream).await.unwrap();
    let mut client = tls.connect(&domain, stream).await.unwrap();

    let get = format!("GET {} HTTP/1.1\r\nConnection: close\r\n\r\n", source);
    client.write_all(get.as_bytes()).await;
    client.flush().await;

    let mut b = String::new();
    let mut reader = BufReader::new(client);

    reader.read_to_string(&mut b).await;
    // for line in reader.lines() {
    //     let billionaire = line.unwrap();
    //     b.push_str(&billionaire);
    // }
    // println!("{}", source);
    b
}
// https://en.wikipedia.org/wiki/Mark_Zuckerberg
pub fn domain(source: &str) -> String {
    match source.starts_with("https") {
        true => {
            let start = source.find("https://").unwrap() + "https://".len();
            let billionaire = &source[start..];
            let end = billionaire.find("/").unwrap();
            let domain = &billionaire[..end];
            domain.to_string()
        }
        false => {
            panic!("provide url starts with https")
        }
    }
}
