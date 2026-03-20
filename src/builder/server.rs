use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

use anyhow::Context;

pub struct Server {
    port: u16,
    dir: PathBuf,
}

impl Server {
    pub fn new(port: u16, dir: &Path) -> anyhow::Result<Self> {
        Ok(Server {
            port,
            dir: dir.canonicalize()?,
        })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let address = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(address.clone()).context("Couldn't bind to port")?;

        println!("Serving files on http://{}", address);

        for stream in listener.incoming() {
            let stream = stream.context("Failed to accept connection")?;
            self.handle_connection(stream)?;
        }

        Ok(())
    }

    fn handle_connection(&self, mut stream: TcpStream) -> anyhow::Result<()> {
        let mut request = String::new();

        let mut buffered = BufReader::new(&stream);
        buffered
            .read_line(&mut request)
            .context("Error reading HTTP request")?;

        let mut parts = request.split_whitespace();
        let method = parts.next().unwrap_or("");
        let mut path = parts.next().unwrap_or("/");

        // Only allow GET
        if method != "GET" {
            let body = "405 Method Not Allowed";
            let response = format!(
                "HTTP/1.1 405 METHOD NOT ALLOWED\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            return Ok(());
        }

        // Drop query string if present: "/file.txt?x=y" -> "/file.txt"
        if let Some(pos) = path.find('?') {
            path = &path[..pos];
        }

        // Default to index.html
        let rel_path = if path == "/" {
            PathBuf::from("index.html")
        } else {
            // Remove leading "/" so it becomes a relative path
            PathBuf::from(path.trim_start_matches('/'))
        };

        // Join with root and canonicalize to resolve "..", symlinks, etc.
        let candidate = self.dir.join(rel_path);
        let Ok(canon) = candidate.canonicalize() else {
            let body = "404 Not Found";
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            return Ok(());
        };

        println!("Serving file: {}", canon.display());

        // SECURITY CHECK: ensure requested path is still inside root
        if !canon.starts_with(self.dir.as_path()) {
            let body = "403 Forbidden";
            let response = format!(
                "HTTP/1.1 403 FORBIDDEN\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            let _ = stream.write_all(response.as_bytes());
            return Ok(());
        }

        // Read and serve file
        match get_path_contents(&canon) {
            Some(contents) => {
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                    contents.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&contents);
            }
            None => {
                let body = "404 Not Found";
                let response = format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes());
            }
        }

        Ok(())
    }
}

fn get_path_contents(path: &Path) -> Option<Vec<u8>> {
    if let Ok(contents) = fs::read(path) {
        return Some(contents);
    }

    let index_file = path.join("index.html");
    if let Ok(contents) = fs::read(&index_file) {
        return Some(contents);
    }

    None
}
