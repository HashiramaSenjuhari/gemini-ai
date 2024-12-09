use std::{
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    panic,
};

use native_tls::TlsConnector;
use regex::Regex;

pub fn search(source: &str) -> String {
    let domain = domain(source);
    let stream = format!("{}:443", domain);
    let tls = TlsConnector::new().unwrap();
    let stream = TcpStream::connect(stream).unwrap();
    let mut client = tls.connect(&domain, stream).unwrap();

    let get = format!("GET {} HTTP/1.1\r\n\r\n", source);
    client.write_all(get.as_bytes());
    client.flush();

    let mut reader = BufReader::new(client);
    // for lines in reader.lines() {
    //     println!("{}", lines.unwrap());
    // }
    // let mut source = String::new();
    let mut body = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line);
        // println!("{}", line);

        let size = usize::from_str_radix(&line.trim(), 16).unwrap();
        println!("{}", size);
        // if size == 0 {
        //     break;
        // }

        // let mut esponse = vec![0; size];
        // reader.read_exact(&mut esponse);
        // let response = String::from_utf8_lossy(&esponse);
        // body.push_str(&response);
        // // println!("{}", response);

        // let mut trail = vec![0; 2];
        // reader.read_exact(&mut trail);
    }
    // println!("{}", source);
    String::new()
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
