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
                println!("{:?}", from_utf8(&data[0..size]));
            }
            true
        },
        Err(_) => {
            println!("Error! :(");
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("from: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| { // For some reason there are pipes here
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
    drop(listener);
}