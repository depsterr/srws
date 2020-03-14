use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::process::exit;
use std::str;
use std::fmt;
use std::env;
use std::fs;

const VERSION:&str = "0.1.0";
const DIRECTORY:&str = "/var/www/html";
const ALLOWSYM:bool = false;

/// This is a function that takes a TcpStream, reads data
/// from it and then sends a response.
fn
handle_client (mut stream: TcpStream) -> Result<(), ()> {
    println!("======= New request =======\n");

    let mut data = [0; 2048];
    stream.read(&mut data).unwrap();

    let data_string = match str::from_utf8(&data) {
        Ok(v) => v,
        Err(_e) => return Err(())
    };

    print!("{}", data_string);

    println!("\n======= End of request =======\n");

    let data_splits: Vec<&str> = data_string.split_whitespace().collect();

    if data_splits[0] != "GET" {
        return Err(())
    }

    let mut filepath = String::from(DIRECTORY);
    filepath.push_str(data_splits[1]);

    /* TODO SANITIZE FILEPATH HERE */

    let metadata = metadata(filepath)?;

    if !metadata.is_ok()
        err404();

    if !(metadata.filetype().is_symlink() && ALLOWSYM)
        err404();
    
    println!("Requesting file {}", filepath);

    return Ok(())
}

fn
main() {
    let args: Vec<String> = env::args().collect();

    let mut address = "127.0.0.1:80";

    if args.len() > 1 {
        address = &args[1];
    }

    println!("Starting srws version {}", VERSION);

    let listener = match TcpListener::bind(address) {
        Ok(v) => v,
        Err(_e) => {
            println!("Unable to start listening on '{}' did you run as root?", address);
            exit(1);
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_e) => continue,
        };
        match handle_client(stream) {
            Ok(v) => v,
            Err(_e) => continue,
        }
    }
}
