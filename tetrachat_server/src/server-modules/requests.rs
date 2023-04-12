use crate::database;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{
    io::{Read, Write},
    net::{Shutdown, TcpStream},
    str::from_utf8,
};

// Define a struct to manage the connected users
pub struct UserList {
    users: Mutex<HashMap<String, Arc<Mutex<TcpStream>>>>,
}

impl UserList {
    pub fn new() -> UserList {
        UserList {
            users: Mutex::new(HashMap::new()),
        }
    }

    fn add_user(&self, uuid: String, stream: &mut TcpStream) {
        println!("Logged in: {}!", uuid);
        let mut users = self.users.lock().unwrap();
        users.insert(uuid, Arc::new(Mutex::new(stream.try_clone().unwrap())));
    }

    pub fn remove_user(&self, uuid: &str) {
        let mut users = self.users.lock().unwrap();
        users.remove(uuid);
    }

    pub fn get_user(&self, uuid: &str) -> Option<Arc<Mutex<TcpStream>>> {
        let users = self.users.lock().unwrap();
        users.get(uuid).cloned()
    }

    fn get_uuid_by_stream(&self, stream: &Arc<Mutex<TcpStream>>) -> Option<String> {
        let users = self.users.lock().unwrap();
        for (uuid, user_stream) in users.iter() {
            if Arc::ptr_eq(stream, user_stream) {
                return Some(uuid.clone());
            }
        }
        None
    }
}

// pub fn handle_client(mut stream: TcpStream, connected_clients: &Arc<Mutex<Vec<User>>>) {
pub fn handle_client(stream: TcpStream, user_list: Arc<UserList>) {
    let mut raw_data = [0 as u8; 512]; // buffer
    loop {
        let mut new_stream = stream.try_clone().unwrap();
        match new_stream.read(&mut raw_data) {
            Ok(size) => {
                if size != 0 {
                    let str_data = String::from(from_utf8(&raw_data).unwrap());
                    let data = str_data.trim_matches(char::from(0));
                    let temp_data = split_data(data);
                    let req_type = temp_data[0].as_str();
                    print!("{:?}", temp_data);
                    match req_type {
                        "ACR" => handle_acr(data, &mut new_stream),
                        "ALR" => handle_alr(data, &mut new_stream, user_list.clone()),
                        "MSR" => handle_msr(data, &mut new_stream, user_list.clone()),
                        "FRR" => handle_frr(data, &mut new_stream),
                        "RMFRR" => handle_rmfrr(data, &mut new_stream),
                        "FETCH" => handle_fetch(data, &mut new_stream),
                        "AFR" => handle_afr(data),
                        "GETAFR" => handle_getafr(data, &mut new_stream),
                        _ => println!("Unknown request"),
                    }
                }
            }
            Err(_) => {
                match new_stream.shutdown(Shutdown::Both) {
                    Ok(_) => {
                        let user = user_list
                            .get_uuid_by_stream(&Arc::new(Mutex::new(new_stream)))
                            .unwrap();
                        println!("{} disconnected!", user);
                        user_list.remove_user(&user);
                    }
                    Err(_e) => {
                        match user_list.get_uuid_by_stream(&Arc::new(Mutex::new(new_stream))) {
                            Some(user) => {
                                println!("{} disconnected!", user);
                                user_list.remove_user(&user);
                            }
                            None => println!("Unknown user disconnected!"),
                        }
                    }
                }
                break;
            }
        }
    }
}


pub fn handle_getafr(data: &str, stream: &mut TcpStream) {
    let split_data = split_data(data);
    let uuid = split_data[1].clone();
    let new_data = database::db_check_for_afr(uuid);
    if new_data != "ERROR" {
        let resp = format!("FAFR-{}", new_data);
        stream.write(resp.as_bytes()).unwrap();
    }
}
pub fn handle_afr(data: &str) {
    let split_data = split_data(data);
    let uuid = split_data[1].clone();
    let uuid_friend = split_data[2].clone();

    database::db_check_afr(uuid, uuid_friend);
}
pub fn handle_fetch(data: &str, stream: &mut TcpStream) {
    let split_data = split_data(data);
    let users = database::db_check_for_frr(split_data[1].to_owned());
    stream.write(users.as_bytes()).unwrap();
}

pub fn handle_acr(data: &str, stream: &mut TcpStream) {
    // account creation request
    // ACR-private_key-nickname
    let split_data: Vec<String> = split_data(data);
    let secret = &split_data[1];
    let nickname = &split_data[2];
    let uuid = database::generate_public();

    database::db_append_new_user(uuid.clone(), secret.to_owned(), nickname.to_owned());

    let response = String::from(format!("ACS-{}", uuid));

    stream.write(response.as_bytes()).unwrap();
}

// pub fn handle_alr(data: &[u8], stream: &mut TcpStream, connected_clients: &Arc<Mutex<Vec<User>>>){
pub fn handle_alr(data: &str, stream: &mut TcpStream, user_list: Arc<UserList>) {
    // account login request
    let hash = &split_data(data)[1];
    let response: String;

    if database::db_check_for_hash(String::from(hash)) {
        let uuid = database::get_public(hash.to_string());
        let username = database::get_username(uuid.clone());

        response = String::from(format!("ALS-{}-{}", uuid, username));
        user_list.add_user(uuid, stream);
    } else {
        response = String::from("ERROR")
    }
    println!("{response}");
    stream.write(response.as_bytes()).unwrap();
}

pub fn handle_rmfrr(data: &str, _stream: &mut TcpStream) {
    let split_data: Vec<String> = split_data(data);
    database::db_rmfrr(split_data[1].clone(), split_data[2].clone());
}

#[allow(unused_must_use)]
pub fn handle_msr(data: &str, _stream: &mut TcpStream, user_list: Arc<UserList>) {
    // message request
    let split_data: Vec<String> = split_data(data);
    let receiver_uuid = &split_data[1];
    let sender_uuid = &split_data[2];
    let msg = String::from(format!("{}-{}", sender_uuid, &split_data[3]));
    if let Some(user_stream) = user_list.get_user(receiver_uuid) {
        let mut new_stream = user_stream.lock().unwrap();
        new_stream.write(msg.as_bytes());
    } else {
        database::send_temp_req(
            String::from("MSR"),
            sender_uuid.to_string(),
            receiver_uuid.to_string(),
            database::get_username(sender_uuid.clone()).to_string(),
        );
    }
}

// pub fn handle_frr(data: &[u8], stream: &mut TcpStream, connected_clients: &Arc<Mutex<Vec<User>>>){
pub fn handle_frr(data: &str, stream: &mut TcpStream) {
    // TODO: finish
    println!("Got friend request!");
    let split_data: Vec<String> = split_data(data);
    let sender_uuid = &split_data[1];
    let receiver_uuid = &split_data[2];
    let sender_username = &split_data[3];
    // println!("sender: {}\nreceiver: {}\nsender username: {}", sender_uuid, receiver_uuid, sender_username);

    database::send_temp_req(
        String::from("FRR"),
        sender_uuid.to_owned(),
        receiver_uuid.to_owned(),
        sender_username.to_owned(),
    );

    stream.write("SUCCESS".as_bytes()).unwrap();
}

fn split_data(data: &str) -> Vec<String> {
    let temp: Vec<String> = data.split("-").map(|s| s.to_owned()).collect();
    return temp;
}
