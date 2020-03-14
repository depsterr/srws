use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::process::exit;
use std::thread;
use std::str;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/* Begin options */
const ADDRESS:&str = "127.0.0.1:80";
const DIRECTORY:&str = "/var/www/html";
const NOTFOUNDPAGE:&str = "/var/www/404.html";
const ALLOWSYM:bool = false;
const MULTIPLEHOSTS:bool = false;
/* End Options */

/// This function takes a TcpStream as an argument which it then reads a
/// HTTP response from to which it will either reply with 404 or 200
/// followed by a 404 page or the correct page.
fn
handle_client (mut stream: TcpStream) -> Result<(), ()> {
    println!("======= Begin Request =======\n");

    let mut data = [0; 2048];
    stream.read(&mut data).unwrap();
let data_string = match str::from_utf8(&data) {
        Ok(v) => v,
        Err(_e) => return Err(())
    };

    print!("{}", data_string);

    println!("\n======= End Request =======");

    let data_splits: Vec<&str> = data_string.split_whitespace().collect();

    let mut filepath = String::from(DIRECTORY);
    if MULTIPLEHOSTS {
        filepath.push_str("/");
        filepath.push_str(data_splits[4]);
    }
    filepath.push_str(data_splits[1]);

    /* TODO SANITIZE FILEPATH HERE */

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

    send_page(stream, &filepath);

    return Ok(())
}

/// Sends the contents of the given file as well as a http 200
/// header to the given stream.
fn
send_page (mut stream: TcpStream, filepath: &str) {
    let page:String = fs::read_to_string(filepath).unwrap().parse().unwrap();
    let mut response = format!("HTTP/1.1 200 OK\nContent-Type: text/html; charset=utf-8\nContent-Length: {}\n\n", page.len());
    response.push_str(&page);
    println!("======= Begin Respone =======\n");
    print!("{}", response);
    println!("\n======= End Respone =======");
    stream.write(response.into_bytes().as_slice()).unwrap();
}

/// Sends the contents of the 404 page as well as a 404
/// header to the given stream.
fn
send_404 (stream: TcpStream) {
    send_page(stream, NOTFOUNDPAGE);
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
