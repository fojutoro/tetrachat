// use dict::{Dict};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use mysql::*;
use mysql::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

// struct User {
//     stream: TcpStream,
//     local_address: String,
//     public_key: String,
//     name: String,
// }

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 512]; // buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            if size != 0 {
                let req_type = from_utf8(&data[0..3]).unwrap();
                match req_type {
                    "ACR" => handle_acr(&data[0..37], &mut stream),
                    "ACC" => println!("nice"),
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
                Err(e) => {
                    println!("User disconnected! :D\n{}", e);
                }
            }
            false
        }
    } {}
}
fn main() {
    let listener = TcpListener::bind("192.168.1.6:8080").unwrap();
    for stream in listener.incoming() {
        // When a client joins for the first time he has to send an ACR - Account creation request. Then we create an account for him.
        match stream {
            Ok(stream) => {
                println!(
                    "local socket address: {:?}\n{:?}",
                    listener.local_addr(),
                    stream.local_addr()
                );
                handle_client(stream);
            }
            Err(e) => {
                println!("{} 2", e);
            }
        }
    }
    drop(listener);
}

fn handle_acr(data: &[u8], stream: &mut TcpStream){
    // println!("handling ACR!");
    let secret = from_utf8(&data[4..20]).unwrap();
    let nickname = from_utf8(&data[21..36]).unwrap();
    let ip: &str = &stream.local_addr().unwrap().ip().to_string();

    let public = generate_public();
    db_append_new_user(public.clone(), secret, nickname, ip);

    let response = String::from(format!("ACS-{}", public));

    stream.write(response.as_bytes()).unwrap();
}

fn generate_public() -> String { // change this later
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return s
}

fn db_append_new_user(public: String, hash: &str, username: &str, ip: &str){
    let url = "mysql://root@localhost:3306/terrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    
    conn.exec_drop(
        r"INSERT INTO users (uuid, privatekey, username, ip)
          VALUES (:uuid, :privatekey, :username, :ip)",
        params! {
            "uuid" => public,
            "privatekey" => hash,
            "username" => username,
            "ip" => ip,
        }
    ).unwrap();
}