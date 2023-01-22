use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    menu();
    match TcpStream::connect("192.168.1.6:8080") {
        Ok(stream) => loop {
            print!(">");
            let msg = get_string();
            let size = msg.len();
            println!("{}", handle_communication(msg, &stream, size));
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn handle_communication(msg: String, mut stream: &TcpStream, size: usize) -> String{
    let msg = msg.as_bytes();
    stream.write(msg).unwrap();

    let mut data = [0 as u8; 512];
    stream.read(&mut data[0..size]).unwrap();
    return from_utf8(&data).unwrap().to_string();
}

fn menu() {
    println!("1. Create account\n 2. Log in.\n  Ctrl + c. Quit");
    let x = get_string();
    if x.trim() == "1" {
        create_acc()
    } else if x.trim() == "2" {
        println!("In development")
    } else {
        println!("Please input a valid option");
        menu();
    }   
}
// TODO: better error handling

fn get_string() -> String {
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    return name;
}


fn create_acc(){
    // send ACR
    println!("Nickname (min: 3, max 16 chars): ");
    
    let name = get_string();
    if name.as_str() == " " || name.len() >= 16 || name.len() <= 3 {
        println!("please input a valid name!");
        create_acc();
    }

    let secret = String::from("1234123412341234");

    println!("Your private key:\n{}", secret);

    let hash = generate_hash(secret);

    let req = String::from(format!("ACR-{}-{}", hash, name));

    let resp = send_req(req);

    let public_id = &resp[3..];

    println!("Your public id: {}", public_id);
    // println!("{}", resp)
}

// TODO: do the hashing
fn generate_hash(secret: String) -> String{
    return secret;
}

fn send_req(req: String) -> String {
    match TcpStream::connect("192.168.1.6:8080") {
        Ok(stream) => {
            let response = handle_communication(req, &stream, 37);
            return response;
        },
        Err(e) => {
            return String::from(format!("{}", e));
        }
    }
}