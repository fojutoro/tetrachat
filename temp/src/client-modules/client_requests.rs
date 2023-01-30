use std::{net::TcpStream, str::from_utf8, io::{Read, Write}};

#[path = "utils.rs"] mod utils;

use rand::Rng;

fn send_req(req: String) -> String {
    match TcpStream::connect("localhost:8080") {
        Ok(stream) => {
            let response = handle_communication(req, &stream);
            return response;
        },
        Err(e) => {
            return String::from(format!("{}", e));
        }
    }
}

pub fn create_acc(){
    // send ACR
    println!("Nickname (min: 3, max 16 chars): ");
    
    let name = utils::get_string();
    if name.as_str() == " " || name.len() >= 16 || name.len() <= 3 {
        println!("please input a valid name!");
        create_acc();
    }

    let secret: i64 = rand::thread_rng().gen_range(1000_0000_0000_0000..=9999_9999_9999_9999);
    let secret: String = secret.to_string();

    println!("Your private key:\n{}", secret);

    let hash = utils::generate_hash(secret);

    let req = String::from(format!("ACR-{}-{}", hash, name));

    let resp = send_req(req);

    let public_id = &resp[4..];

    println!("Your public id: {}", public_id);
    menu();
}

pub fn log_in() {
    println!("Please input your private key: ");
    let secret = utils::generate_hash(utils::get_string().trim().to_string());
    let req = String::from(format!("ALR-{}", secret));
    let resp = send_req(req);

    if &resp.as_str()[0..3] == "ALS" {
        handle_comms();
    } else {
        println!("Invalid key!");
        menu();
    }
}


fn handle_comms() {
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            loop {
                println!("Who do you want to send the message to?");
                let reciever_uuid = utils::get_string();
                let req = format!("MSR-{}-{}", reciever_uuid, utils::get_string());
                stream.write(req.as_bytes()).unwrap();
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn handle_communication(msg: String, mut stream: &TcpStream) -> String{
    let msg = msg.as_bytes();
    stream.write(msg).unwrap();

    let mut data = [0 as u8; 512];
    stream.read(&mut data).unwrap();
    return from_utf8(&data).unwrap().to_string();
}

pub fn menu() {
    println!("1. Create account\n 2. Log in.\n  Ctrl + c. Quit");
    let x = utils::get_string();
    if x.trim() == "1" {
        create_acc()
    } else if x.trim() == "2" {
        log_in()
    } else {
        println!("Please input a valid option");
        menu();
    }   
}