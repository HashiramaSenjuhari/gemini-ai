use std::{
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
};

use dotenv::dotenv;
use http_scrap::Response;
use native_tls::TlsConnector;
use serde::Deserialize;

pub struct File<'file, FolderNameState, FileTypeState, FileNameState, EnvState> {
    env_variable: &'file str,
    folder_name: &'file str,
    file_type: &'file str,
    file_name: &'file str,
    foldernamestate: std::marker::PhantomData<FolderNameState>,
    filenamestate: std::marker::PhantomData<FileNameState>,
    filetypestate: std::marker::PhantomData<FileTypeState>,
    envstate: std::marker::PhantomData<EnvState>,
}

pub struct FolderNamePresent;
pub struct FolderNameNotPresent;

pub struct FileTypePresent;
pub struct FileTypeNotPresent;

pub struct FileNamePresent;
pub struct FileNameNotPresent;

pub struct EnvPresent;
pub struct EnvNotPresent;

impl<'file>
    File<'file, FolderNameNotPresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent>
{
    pub fn new() -> Self {
        Self {
            folder_name: "",
            file_type: "",
            file_name: "",
            env_variable: "",
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
    pub fn set_name(
        mut self,
        name: &'file str,
    ) -> File<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent> {
        self.folder_name = name;
        File {
            folder_name: self.folder_name,
            file_type: self.file_type,
            file_name: self.file_name,
            env_variable: &self.env_variable,
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
}

impl<'file> File<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent> {
    pub fn env(
        mut self,
        env_variable: &'file str,
    ) -> File<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvPresent> {
        self.env_variable = env_variable;
        File {
            folder_name: self.folder_name,
            file_type: self.file_type,
            file_name: self.file_name,
            env_variable: &self.env_variable,
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
}

impl<'file> File<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvPresent> {
    pub fn file_path(
        mut self,
        name: &'file str,
    ) -> File<'file, FolderNamePresent, FileNamePresent, FileTypeNotPresent, EnvPresent> {
        self.file_name = name;
        File {
            folder_name: self.folder_name,
            file_type: self.file_type,
            file_name: self.file_name,
            env_variable: &self.env_variable,
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
}

impl<'file> File<'file, FolderNamePresent, FileNamePresent, FileTypeNotPresent, EnvPresent> {
    pub fn file_type(
        mut self,
        mime_type: &'file str,
    ) -> File<'file, FolderNamePresent, FileNamePresent, FileTypePresent, EnvPresent> {
        self.file_type = mime_type;
        File {
            folder_name: self.folder_name,
            file_type: self.file_type,
            file_name: self.file_name,
            env_variable: &self.env_variable,
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
}

impl<'file> File<'file, FolderNamePresent, FileNamePresent, FileTypePresent, EnvPresent> {
    pub fn output(self) -> Result<Billionaire, serde_json::Error> {
        dotenv().unwrap();
        let env = env::var(self.env_variable).unwrap();
        let file = fs::metadata(self.file_name).unwrap();
        let filee = fs::read(self.file_name).unwrap();
        Self::create_folder(&env, file.len(), &self.file_type, filee, self.folder_name)
        // Self::upload_audio("/upload/v1beta/files?key=AIzaSyAxwL368iXBuVu8FlKi0Ze5JvagZbPi9so&upload_id=AFiumC7nWrL2Bm7-yHprTrVS5XoqLxJsdwQnAn085m33xwYgrzfuLrFJg0I0ApVpxGH7uqKQTq-YhZdFnalkUihx9yR8YBL9w4ppSxZFzieUYa-w3w&upload_protocol=resumable", file.len(), filee)
    }
    fn create_folder(
        env: &str,
        file_size: u64,
        file_type: &str,
        filee: Vec<u8>,
        folder: &str,
    ) -> Result<Billionaire, serde_json::Error> {
        let file = format!(
            r#"{{
            "file": {{
              "display_name": "{}"
        }}
        }}"#,
            folder
        );
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut connect = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let post = format!(
          "POST /upload/v1beta/files?key={} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
            X-Goog-Upload-Protocol: resumable\r\n\
            X-Goog-Upload-Command: start\r\n\
            X-Goog-Upload-Header-Content-Length: {}\r\n\
            X-Goog-Upload-Header-Content-Type: {}\r\n\
            Content-Type: application/json\r\n\
            Content-length: {}\r\n\r\n\
        {}",
          env, file_size,file_type, file.len(), file
      );
        connect.write_all(post.as_bytes());
        connect.flush();

        let mut buffer = BufReader::new(connect);
        let mut response = String::new();
        for line in buffer.by_ref().lines() {
            let line = line.unwrap();
            println!("{}",line);
            if line.trim().starts_with("X-Goog-Upload-URL") {
                let line = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .trim_start_matches("https://generativelanguage.googleapis.com");
                response.push_str(line);
            }
            if line.trim().starts_with("X-Goog-Upload-Control-URL") {
                break;
            }
        }
        // buffer.into_inner().shutdown();
        println!("{}", response);
        Self::upload_audio(&response, file_size, filee, file_type)
    }

    fn upload_audio(
        route: &str,
        file_size: u64,
        file: Vec<u8>,
        r#type: &str,
    ) -> Result<Billionaire, serde_json::Error> {
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut connect = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();
        let post = format!(
            "POST {} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
        Content-Length: {}\r\n\
        Content-Type: {}\r\n\
        X-Goog-Upload-Offset: 0\r\n\
        X-Goog-Upload-Command: upload,finalize\r\n\r\n\
        {:?}",
            route, file_size, r#type, file
        );
        // println!("{}", post);
        // println!("{}", post);
        connect.write_all(post.as_bytes());
        connect.flush();

        let mut buffer = [0; 1024];
        connect.read(&mut buffer);

        let bo = String::from_utf8_lossy(&buffer);
        let content = Response::new(&bo);
        println!("{:?}", content.content());
        let get = content.content();
        let start = get
            .trim_start_matches("{")
            .trim()
            .trim_start_matches("\"file\": ")
            .trim()
            .trim_end_matches("}");
        println!("{}", start);
        let post = serde_json::from_str::<Billionaire>(start);
        // println!("{:?}", post);
        post
    }
}

#[derive(Debug, Deserialize)]
pub struct Billionaire {
    pub name: String,
    pub displayName: String,
    pub mimeType: String,
    pub sizeBytes: String,
    pub createTime: String,
    pub updateTime: String,
    pub expirationTime: String,
    pub sha256Hash: String,
    pub uri: String,
    pub state: String,
}
