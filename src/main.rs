use std::net::{TcpListener, TcpStream};
use std::sync::{Mutex, Arc};
use std::io::prelude::*;
use std::time::Duration;
use std::thread;
use std::char;

const MAX_CLIENTS:i32 = 50;

fn
handle_client (mut stream: TcpStream, client_lock: Arc<Mutex<i32>>) {
    let mut clients = client_lock.lock().unwrap();
    let mut data = [0; 128];
    stream.read(&mut data).unwrap();
    println!("Request #{}:", *clients);
    for x in data.iter() {
        let c:char = char::from_u32(*x as u32).unwrap();
        print!("{}", c);
    }
    *clients -= 1;
    println!("exiting thread");
}

fn
main() {
    // add some sort of arg handeling here
    println!("Starting srws version 0.1.0");

    let client_lock = Arc::new(Mutex::new(0));
    let clients = Arc::clone(&client_lock);
    
    let listener = TcpListener::bind("localhost:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(10)));

        let clients = clients.lock().unwrap();

        if *clients < MAX_CLIENTS {
            thread::spawn( move || handle_client(stream, Arc::clone(&client_lock)));
        } else {
            while *clients == MAX_CLIENTS {
                thread::sleep(Duration::from_secs(1));
            }
            thread::spawn( move || handle_client(stream, Arc::clone(&client_lock)));
        }
    }
}
