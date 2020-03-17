use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::exit;
use std::str;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread;

use readconfig::Configuration;

struct Options {
    address: String,
    directory: String,
    not_found_page: String,
    allow_sym: bool,
    multiple_hosts: bool,
}

impl Clone for Options {
    fn clone(&self) -> Options {
        Options { address: self.address.clone(), directory: self.directory.clone(), not_found_page: self.not_found_page.clone(), allow_sym: self.allow_sym.clone(), multiple_hosts: self.multiple_hosts.clone(), }
    }
}

/// The program version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The max size for HTTP requests, in bytes
const MAX_REQUEST_SIZE:usize = 4096;


/// This function takes a TcpStream as an argument which it then reads a
/// HTTP request from to which it will either reply with a 404 or 200
/// response and a corresponding html page.
fn
handle_client (mut stream: TcpStream, options: Arc<RwLock<Options>> ) -> Result<(), ()> {
    println!("======= Begin Request =======\n");

    let mut data = [0; MAX_REQUEST_SIZE];
    
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

    let mut filepath = String::from(&options.read().unwrap().directory);
    if options.read().unwrap().multiple_hosts {
        filepath.push('/');
        filepath.push_str(data_splits[4]);
    }
    filepath.push_str(data_splits[1]);

    let mut metadata = match fs::metadata(filepath.clone()) {
        Ok(v) => v,
        Err(_e) => {
            send_page(stream, &options.read().unwrap().not_found_page, "404 Not Found", "text/html");
            return Ok(())
        },
    };

    if metadata.is_dir() {
        filepath.push_str("/index.html");
        metadata = match fs::metadata(filepath.clone()) {
            Ok(v) => v,
            Err(_e) => {
                send_page(stream, &options.read().unwrap().not_found_page, "404 Not Found", "text/html");
                return Ok(())
            },
        };
    }

    if metadata.file_type().is_symlink() && !options.read().unwrap().allow_sym {
        send_page(stream, &options.read().unwrap().not_found_page, "404 Not Found", "text/html");
        return Ok(())
    }

    let extension = Path::new(&filepath).extension();

    if extension == None {
        send_page(stream, &filepath, "200 OK", "text");
        return Ok(())
    }

    match extension.unwrap().to_str().unwrap() {
        "html" => send_page(stream, &filepath, "200 OK", "text/html; charset=utf-8"),
        "jpg" | "jpeg" => send_page(stream, &filepath, "200 OK", "image/jpeg"),
        "png" => send_page(stream, &filepath, "200 OK", "image/png"),
        "css" => send_page(stream, &filepath, "200 OK", "text/css"),
        "js" => send_page(stream, &filepath, "200 OK", "text/javascript"),
        "json" => send_page(stream, &filepath, "200 OK", "application/json"),
        "mp3" => send_page(stream, &filepath, "200 OK", "audio/mpeg"),
        "svg" => send_page(stream, &filepath, "200 OK", "image/svg+xml"),
        "ico" => send_page(stream, &filepath, "200 OK", "image/vnd.microsoft.icon"),
        "bmp" => send_page(stream, &filepath, "200 OK", "image/bmp"),
        "gif" => send_page(stream, &filepath, "200 OK", "audio/gif"),
        _ => send_page(stream, &filepath, "200 OK", "text/html; charset=utf-8"),
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
    let header = format!("HTTP/1.1 {}\nContent-Type: {}\nX-Content-Type-Options: nosniff\nContent-Length: {}\n\n", status, contenttype, file.len());
    let mut response = Vec::from(header.into_bytes().as_slice());
    response.append(&mut file);
    stream.write(&response).unwrap();
}

fn
main() {
    println!("Starting srws version {}", VERSION);

    /* Read options from options file */
    let cfg = Configuration::new(&["/etc/srws.conf"]);

    let options = Arc::new( RwLock::new( Options {
        address: match cfg.get_option("address") {
            Ok(v) => v,
            Err(_e) => "0.0.0.0:80".to_string(),
        },
        directory: match cfg.get_option("directory") {
            Ok(v) => v,
            Err(_e) => "/var/www/html".to_string(),
        },
        not_found_page: match cfg.get_option("not_found_page") {
            Ok(v) => v,
            Err(_e) => "/var/www/404.html".to_string(),
        },
        allow_sym: match bool::from_str(&match cfg.get_option("allow_sym") { Ok(v) => v, Err(_e) => "false".to_string()}) {
            Ok(v) => v,
            Err(_e) => false,
        },
        multiple_hosts: match bool::from_str(&match cfg.get_option("multiple_hosts") { Ok(v) => v, Err(_e) => "false".to_string()}) {
            Ok(v) => v,
            Err(_e) => false,
        },
    } ) );
    

    let listener = match TcpListener::bind(&options.read().unwrap().address) {
        Ok(v) => v,
        Err(_e) => {
            println!("Unable to start listening on '{}', do you have the needed permissions?", &options.read().unwrap().address);
            exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_e) => continue,
        };
        let opt = options.clone();
        thread::spawn( move || {
            handle_client(stream, opt)
        });
    }
}
