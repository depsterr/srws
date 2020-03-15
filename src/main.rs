use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::process::exit;
use std::path::Path;
use std::fs::File;
use std::thread;
use std::str;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The address that the server will listen on. The default value covers
/// all connections on port 80
const ADDRESS:&str = "0.0.0.0:80";
/// The base directory for the webpage.
const DIRECTORY:&str = "/var/www/html";
/// The page to show in case of a 404 Not Found error
const NOTFOUNDPAGE:&str = "/var/www/404.html";
/// Allow opening symlinks? (Note that symlink paths are not blocked by
/// this option.
const ALLOWSYM:bool = false;
/// If set to true, the server will serve webpages from a subdirectory
/// with the name of the host. For example, if you were to connect to
/// examplewebsite.com then the server would use the folder
/// /var/www/html/examplewebsite.com/ as it's base directory. This is
/// useful if you want to host multiple website on one server.
const MULTIPLEHOSTS:bool = false;
/// The max amount of bytes to be able to read as a http request.
const MAXREQUESTSIZE:usize = 4096;

/// This function takes a TcpStream as an argument which it then reads a
/// HTTP request from to which it will either reply with a 404 or 200
/// response and a corresponding html page.
fn
handle_client (mut stream: TcpStream) -> Result<(), ()> {
    println!("======= Begin Request =======\n");

    let mut data = [0; MAXREQUESTSIZE];
    
    match stream.read(&mut data) {
        Ok(v) => v,
        Err(_e) => return Err(()),
    };

    let data_string = match str::from_utf8(&data) {
        Ok(v) => v,
        Err(_e) => return Err(())
    };

    print!("{}", data_string);

    println!("\n======= End Request =======");

    let data_splits: Vec<&str> = data_string.split_whitespace().collect();

    if data_splits.len() < 5 {
        return Err(());
    }

    let mut filepath = String::from(DIRECTORY);
    if MULTIPLEHOSTS {
        filepath.push('/');
        filepath.push_str(data_splits[4]);
    }
    filepath.push_str(data_splits[1]);

    let mut metadata = match fs::metadata(filepath.clone()) {
        Ok(v) => v,
        Err(_e) => {
            send_404(stream);
            return Ok(())
        },
    };

    if metadata.is_dir() {
        filepath.push_str("/index.html");
        metadata = match fs::metadata(filepath.clone()) {
            Ok(v) => v,
            Err(_e) => {
                send_404(stream);
                return Ok(())
            },
        };
    }

    if metadata.file_type().is_symlink() && !ALLOWSYM {
        send_404(stream);
        return Ok(())
    }

    let extension = Path::new(&filepath).extension();

    if extension == None {
        send_page(stream, &filepath, "200 OK", "text/html");
        return Ok(())
    }

    match extension.unwrap().to_str().unwrap() {
        "jpg" | "png" => send_page(stream, &filepath, "200 OK", "image"),
        _ => send_page(stream, &filepath, "200 OK", "text/html"),
    }
    

    return Ok(())
}

/// Takes a TcpStream, a filepath and a status, it then sends
/// a HTTP response to the stream with status as the status
/// and the contents of the file located at the filepath
/// as a html body. This function does not sanitize input.
fn
send_page (mut stream: TcpStream, filepath: &str, status: &str, contenttype: &str) {
    let mut fptr = File::open(filepath).unwrap();
    let mut file = Vec::new();
    fptr.read_to_end(&mut file).unwrap();
    let header = format!("HTTP/1.1 {}\nContent-Type: {} charset=utf-8\nContent-Length: {}\n\n", status, contenttype, file.len());
    let mut response = Vec::from(header.into_bytes().as_slice());
    response.append(&mut file);
    stream.write(&response).unwrap();
}

/// Sends a 404 response to the given TcpStream.
fn
send_404 (stream: TcpStream) {
    send_page(stream, NOTFOUNDPAGE, "404 Not Found", "text/html");
}

fn
main() {
    println!("Starting srws version {}", VERSION);

    let listener = match TcpListener::bind(ADDRESS) {
        Ok(v) => v,
        Err(_e) => {
            println!("Unable to start listening on '{}', do you have the needed permissions?", ADDRESS);
            exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_e) => continue,
        };
        thread::spawn( || handle_client(stream));
    }
}
