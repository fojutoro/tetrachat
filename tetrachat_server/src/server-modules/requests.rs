
use std::{str::from_utf8, net::{TcpStream, Shutdown}, io::{Write, Read}};

#[path = "database.rs"] mod database;

pub fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 512]; // buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size != 0 {
                println!("{}", from_utf8(&data).unwrap());
                let req_type = from_utf8(&data[0..3]).unwrap();
                println!("{}", req_type);
                match req_type {
                    "ACR" => handle_acr(&data, &mut stream),
                    "ALR" => handle_alr(&data, &mut stream),
                    "MSR" => handle_msr(&data, &mut stream),
                    _ => println!("Uknown request"),
                }
            }
            true
        }
        Err(_) => {
            match stream.shutdown(Shutdown::Both) {
                Ok(_) => {
                    println!("User disconnected!");
                }
                Err(_e) => {
                    println!("User@{} disconnected! :D\n", stream.local_addr().unwrap().ip());
                }
            }
            false
        }
    } {}
}

pub fn handle_acr(data: &[u8], stream: &mut TcpStream){
    let secret = from_utf8(&data[4..20]).unwrap();
    let nickname = from_utf8(&data[21..36]).unwrap();
    let ip: &str = &stream.local_addr().unwrap().ip().to_string();
    let public = database::generate_public();

    database::db_append_new_user(public.clone(), secret, nickname, ip);

    let response = String::from(format!("ACS-{}", public));

    stream.write(response.as_bytes()).unwrap();
}

pub fn handle_alr(data: &[u8], stream: &mut TcpStream){
    let hash = from_utf8(&data[4..]).unwrap();
    let response: String;

    match database::db_check_for_hash(String::from(hash)){
        true => response = String::from("ALS"),
        false => response = String::from("ERROR"),
    };
    println!("{response}");
    // let new_user = database::get_user(&stream, hash);
    stream.write(response.as_bytes()).unwrap();
}
// TODO: Take care of the previous buffer
pub fn handle_msr(data: &[u8], _stream: &mut TcpStream){
    let reciever_uuid = String::from(from_utf8(&data[4..11]).unwrap());
    let msg = String::from(from_utf8(&data[12..]).unwrap());
    println!("{reciever_uuid}{msg}");
    // println!("{}> {}", stream.local_addr().unwrap().ip(), msg);
}
