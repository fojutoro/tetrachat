use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
fn main() {
    loop {
        match TcpStream::connect("localhost:8080") {
            Ok(mut stream) => {
                print!("> ");
                let mut msg = String::new();
                match std::io::stdin().read_line(&mut msg) {
                    Ok(_) => {
                        let msg = msg.as_bytes();
                        stream.write(msg).unwrap();

                        let mut data = [0 as u8; 512];
                        match stream.read(&mut data) {
                            Ok(size) => {
                                if &data == msg {
                                    println!("Reply is ok!");
                                } else {
                                    let text = from_utf8(&data[0..size]).unwrap();
                                    println!("recieved: {}", text);
                                }
                            },
                            Err(e) => {
                                println!("Failed to receive data: {}", e);
                            }
                }
                    }
                    Err(e) => {
                        println!("Couldn't read message. :(\nerror: {}", e);
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}