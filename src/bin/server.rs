use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 512]; // buffer
    while match stream.read(&mut data){
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            if size != 0 {
                println!("{}", from_utf8(&data[0..size]).unwrap());
            }
            true
        },
        Err(_) => {
            match stream.shutdown(Shutdown::Both){
                Ok(_) => {
                    println!("User disconnected!");
                },
                Err(e) => {
                    println!("User disconnected! :D\n{}", e);
                }
            }
            false
        }
    } {}
}
fn main(){
    let listener = TcpListener::bind("192.168.1.6:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| { // For some reason there are pipes here
                    println!("from: {}", stream.peer_addr().unwrap());
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("{} 2", e);
            }
        }
    }
    drop(listener);
}