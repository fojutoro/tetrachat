use std::net::TcpStream;

use mysql::{params, Pool};
use mysql::prelude::{Queryable};
use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct User<'a> {
    stream: &'a TcpStream,
    public_key: String,
    name: String,
}

pub fn db_append_new_user(public: String, hash: &str, username: &str, ip: &str){
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
    println!("{}, Created new account!", ip);
}

pub fn generate_public() -> String { // change this later
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return s
}
pub fn db_check_for_hash(hash: String) -> bool{
    let url = "mysql://root@localhost:3306/terrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let q_otp: Option<String> = conn.query_first(
        format!("SELECT COUNT(*) FROM users WHERE privatekey = \"{}\";", hash)
    ).unwrap();
    
    let no_hashes: usize = q_otp.unwrap().parse().unwrap();

    return no_hashes > 0;
}

pub fn get_user<'a>(stream: &'a TcpStream, hash: &'a str) -> User<'a> {
    let url = "mysql://root@localhost:3306/terrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let mut res = conn.query_map(
        format!("select uuid, username from users where privatekey = {}", hash),
        |(uuid, username),| 
        User {
                stream: &stream,
                public_key: uuid,
                name: username,
            }
    ).expect("Query failed.");
    let new_user = res.remove(0);
    println!("--------\nNEW USER LOGGED IN!\nIP: {}\nUUID: {}\nNAME: {}\n--------", new_user.stream.local_addr().unwrap().ip(), new_user.public_key, new_user.name);
    return new_user;
}
