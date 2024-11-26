use std::{
    env, fs,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use dotenv::dotenv;
use native_tls::TlsConnector;

pub enum Type {
    Pdf,
}
pub struct VideoFile<'file, FolderNameState, FileTypeState, FileNameState, EnvState> {
    env_variable: &'file str,
    folder_name: &'file str,
    file_type: Type,
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
    VideoFile<'file, FolderNameNotPresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent>
{
    pub fn new() -> Self {
        Self {
            folder_name: "",
            file_type: Type::Pdf,
            file_name: "",
            env_variable: "",
            foldernamestate: std::marker::PhantomData,
            filenamestate: std::marker::PhantomData,
            filetypestate: std::marker::PhantomData,
            envstate: std::marker::PhantomData,
        }
    }
    pub fn dir_name(
        mut self,
        name: &'file str,
    ) -> VideoFile<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent>
    {
        self.folder_name = name;
        VideoFile {
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

impl<'file>
    VideoFile<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvNotPresent>
{
    pub fn env(
        mut self,
        env_variable: &'file str,
    ) -> VideoFile<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvPresent>
    {
        self.env_variable = env_variable;
        VideoFile {
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

impl<'file>
    VideoFile<'file, FolderNamePresent, FileNameNotPresent, FileTypeNotPresent, EnvPresent>
{
    pub fn file_path(
        mut self,
        name: &'file str,
    ) -> VideoFile<'file, FolderNamePresent, FileNamePresent, FileTypeNotPresent, EnvPresent> {
        self.file_name = name;
        VideoFile {
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

impl<'file> VideoFile<'file, FolderNamePresent, FileNamePresent, FileTypeNotPresent, EnvPresent> {
    pub fn file_type(
        mut self,
        mime_type: Type,
    ) -> VideoFile<'file, FolderNamePresent, FileNamePresent, FileTypePresent, EnvPresent> {
        self.file_type = mime_type;
        VideoFile {
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

impl<'file> VideoFile<'file, FolderNamePresent, FileNamePresent, FileTypePresent, EnvPresent> {
    pub fn output(self) -> () {
        dotenv().unwrap();
        let env = env::var(self.env_variable).unwrap();
        let file = fs::metadata(self.file_name).unwrap();
        let filee = fs::read(self.file_name).unwrap();
        match self.file_type {
            Type::Pdf => {
                Self::create_folder(&env, file.len(), "video/mp4", filee, self.folder_name)
            }
        }
    }
    fn create_folder(
        env: &str,
        file_size: u64,
        file_type: &str,
        filee: Vec<u8>,
        folder: &str,
    ) -> () {
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

        let buffer = BufReader::new(connect);
        let mut response = String::new();
        for line in buffer.lines() {
            let line = line.unwrap();
            // println!("{}", line);
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
        Self::upload_pdf(&response, file_size, filee, env)
    }

    fn upload_pdf(route: &str, file_size: u64, file: Vec<u8>, env: &str) {
        let mut is_not_active = true;
        while is_not_active {
            println!("{}", "1");
            let active = Self::upload(route, file_size, file.clone(), is_not_active);
            // is_not_active = active
        }
    }
    fn upload(route: &str, file_size: u64, file: Vec<u8>, mut active: bool) -> bool {
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut connect = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();
        let post = format!(
            "POST {} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
      Content-Length: {}\r\n\
      X-Goog-Upload-Offset: 0\r\n\
      X-Goog-Upload-Command: upload,finalize\r\n\r\n\
      {:?}",
            route, file_size, file
        );
        // println!("{}", post);
        connect.write_all(post.as_bytes());
        connect.flush();

        let buffer = BufReader::new(connect);
        let mut url = String::new();
        for lines in buffer.lines() {
            let line = lines.unwrap();
            println!("{}", line);
            if line.trim().starts_with("\"uri\"") {
                let uri = line.split(": ").nth(1).unwrap().trim();
                url.push_str(uri);
            }
            if line.trim().starts_with("\"state\"") {
                let uri = line.split(": ").nth(1).unwrap().trim();
                if uri == "\"ACTIVE\"" {
                    println!("{}", url);
                    active = false;
                    break;
                } else {
                    active = true;
                    break;
                }
            }
        }
        active
    }
}
