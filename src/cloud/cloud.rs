use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use base64::encode;
use http_scrap::Response;
use native_tls::TlsConnector;
use serde::Deserialize;

pub fn create_cloud_gemini_and_get_upload_uri(video: &str) -> String {
    let metadata = fs::metadata(video).unwrap();
    let len = metadata.len();

    let cloud_gemini: &str = r#"{
    "file":{
      "display_name":"billionaire"
    }
  }"#;

    let tls = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
    let mut connect = tls
        .connect("generativelanguage.googleapis.com", stream)
        .unwrap();

    let response = format!("POST /upload/v1beta/files?key=AIzaSyDJXrc6CmpwX5TwF2R53ro_yd18dQ_gMNM HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
    X-Goog-Upload-Protocol: resumable\r\n\
    X-Goog-Upload-Command: start\r\n\
    X-Goog-Upload-Header-Content-Length: {}\r\n\
    X-Goog-Upload-Header-Content-Type: video/mp4\r\n\
    Content-Type: application/json\r\n\
    Content-Length: {}\r\n\r\n\
    {}
    ",
        len,
        cloud_gemini.len(),
        cloud_gemini
    );

    // println!("{}", response);start

    connect.write_all(response.as_bytes());
    connect.flush();

    let mut response = String::new();
    let mut buffer = BufReader::new(connect);
    for reader in buffer.lines() {
        let reader = reader.unwrap();
        if reader.starts_with("X-Goog-Upload-URL") {
            response.push_str(&reader);
        }
        println!("{}", reader);
        if reader.starts_with("X-Goog-Upload-Control-URL") {
            break;
        }
    }
    let upload_uri = response.split(": ").nth(1).unwrap();
    // println!("{}", upload_uri);
    upload_video_to_gemini_cloud(upload_uri, video);
    String::new()
}

pub fn upload_video_to_gemini_cloud(upload_uri: &str, video: &str) {
    let path = upload_uri;
    let path = path.trim_start_matches("https://generativelanguage.googleapis.com");
    // println!("{}", upload_uri);
    let metadata = fs::metadata(video).unwrap();

    let vidoe = fs::read(video).unwrap();
    let bytes = encode(vidoe);

    let tls = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
    let mut connect = tls
        .connect("generativelanguage.googleapis.com", stream)
        .unwrap();

    let response = format!(
        "POST {} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
  Content-Type: multipart/form-data\r\n\
  Content-Length: {}\r\n\
  X-Goog-Upload-Offset: 0\r\n\
  X-Goog-Upload-Command: upload,finalize\r\n\r\n\
  {}
  ",
        upload_uri,
        metadata.len(),
        bytes
    );

    connect.write_all(response.as_bytes());
    connect.flush();

    let mut buffer = [0; 1024];
    connect.read(&mut buffer);

    let response = String::from_utf8_lossy(&buffer);
    let respons = Response::new(&response);
    let response = respons
        .content()
        .trim()
        .trim_start_matches("{")
        .trim_end_matches("}")
        .split("\"file\":")
        .nth(1)
        .unwrap()
        .trim();
    println!("{}", response);
    let file = serde_json::from_str::<Files>(response).unwrap();
    println!("{:?}", file);

    println!("{}", "sent");
    // check_state(file);
}

#[derive(Debug, Deserialize)]
struct Files {
    name: String,
    displayName: String,
    mimeType: String,
    sizeBytes: String,
    createTime: String,
    updateTime: String,
    expirationTime: String,
    sha256Hash: String,
    uri: String,
    state: String,
}

pub fn check_state() {
    let tls = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
    let mut connect = tls
        .connect("generativelanguage.googleapis.com", stream)
        .unwrap();

    let response = format!("GET /v1beta/files/75y4otkc91bu?key=AIzaSyDJXrc6CmpwX5TwF2R53ro_yd18dQ_gMNM HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\r\n");

    connect.write_all(response.as_bytes());
    connect.flush();

    let mut buffer = [0; 1024];
    connect.read(&mut buffer);

    let response = String::from_utf8_lossy(&buffer);
    println!("{}", response);
}
