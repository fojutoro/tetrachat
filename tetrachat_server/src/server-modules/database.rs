use mysql::{params, Pool, Row};
use mysql::prelude::{Queryable};
use rand::Rng;
use rand::distributions::Alphanumeric;

pub fn db_append_new_user(public: String, hash: String, username: String){
    let username = username.chars().filter(|c| c.is_alphabetic() || c.is_numeric()).collect::<String>();
    for letter in username.chars(){
        print!("{} ",letter);
    }
    print!("uuid: {}\nprivatekey: {}\nusername: {}",
    public, hash, username);
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    
    conn.exec_drop(
        r"INSERT INTO users (uuid, privatekey, username)
            VALUES (:uuid, :privatekey, :username)",
        params! {
            "uuid" => public,
            "privatekey" => hash,
            "username" => username,
        }
    ).unwrap();
}

pub fn generate_public() -> String { // change this later
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    return s
}

pub fn db_rmfrr(uuid: String, uuid_friend: String) {
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    println!("It's happening");
    #[allow(unused_variables)]
    let q_otp: Option<String> = conn.query_first(
        format!("DELETE FROM temp_requests WHERE sender = \"{}\" AND receiver = \"{}\" AND req_type = \"FRR\";", uuid_friend, uuid)
    ).unwrap();
}
pub fn db_check_afr(uuid: String, uuid_friend: String) {
    send_temp_req(String::from("AFR"), uuid, uuid_friend, String::from("NULL"));
}

pub fn db_check_for_hash(hash: String) -> bool{
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let q_otp: Option<String> = conn.query_first(
        format!("SELECT COUNT(*) FROM users WHERE privatekey = \"{}\";", hash)
    ).unwrap();
    
    let no_hashes: usize = q_otp.unwrap().parse().unwrap();

    return no_hashes > 0;
}

#[allow(dead_code)]
pub fn db_check_for_uuid(nickname: String) -> bool{
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let q_otp: Option<String> = conn.query_first(
        format!("SELECT COUNT(*) FROM users WHERE uuid = \"{}\";", nickname)
    ).unwrap();
    
    let no_users: usize = q_otp.unwrap().parse().unwrap();

    return no_users > 0;
}

pub fn get_public(hash: String) -> String{
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let q_otp: Option<String> = conn.query_first(
        format!("SELECT uuid FROM users WHERE privatekey = \"{}\";", hash)
    ).unwrap();
    
    let uuid: String = q_otp.unwrap().to_string();
    return uuid;
}

pub fn get_username(uuid: String) -> String {
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let q_otp: Option<String> = conn.query_first(
        format!("SELECT username FROM users WHERE uuid = \"{}\";", uuid)
    ).unwrap();
    
    let name: String = q_otp.unwrap().to_string();
    return name;
}

pub fn send_temp_req(req: String, sender: String, receiver: String, sender_name: String){
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    println!("{}\n{}\n{}", req, sender, receiver);
    println!("len of receiver: {}", receiver.len());
    conn.exec_drop(
        r"INSERT INTO temp_requests(req_type, sender, receiver, sender_name) VALUES (?, ?, ?, ?)",( req, sender, receiver, sender_name)).unwrap();
}
pub fn db_check_for_afr(uuid: String) -> String {
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let row: Option<Row> = conn.query_first(
        format!(r"SELECT sender, sender_name FROM temp_requests WHERE receiver = '{}' AND req_type = 'AFR';", uuid)).unwrap();
    println!("row: {:?}", row);
    
    if let Some(row) = row {
        let sender: String = row.get(0).unwrap();
        let sender_name: String = row.get(1).unwrap();
        println!("{}-{}", sender, sender_name);
        return format!("{}-{}", sender, sender_name);
    } else {
        return String::from("ERROR");
    }
}
pub fn db_check_for_frr(uuid: String) -> String {
    let url = "mysql://root@localhost:3306/Tetrachat";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();
    let row: Option<Row> = conn.query_first(
        format!(r"SELECT sender, sender_name FROM temp_requests WHERE receiver = '{}' AND req_type = 'FRR';", uuid)).unwrap();
    println!("row: {:?}", row);
    
    if let Some(row) = row {
        let sender: String = row.get(0).unwrap();
        let sender_name: String = row.get(1).unwrap();
        println!("{}-{}", sender, sender_name);
        return format!("{}-{}", sender, sender_name);
    } else {
        return String::from("ERROR");
    }
}
// pub fn get_user<'a>(stream: &'a TcpStream, hash: &'a str) -> User<'a> {
//     let url = "mysql://root@localhost:3306/Tetrachat";
//     let pool = Pool::new(url).unwrap();

//     let mut conn = pool.get_conn().unwrap();
//     let mut res = conn.query_map(
//         format!("select uuid, username from users where privatekey = {}", hash),
//         |(uuid, username),| 
//         // User {
//         //         stream: &stream,
//         //         public_key: uuid,
//         //         name: username,
//         //     }
//     ).expect("Query failed.");
//     let new_user = res.remove(0);
//     println!("--------\nNEW USER LOGGED IN!\nIP: {}\nUUID: {}\nNAME: {}\n--------", new_user.stream.local_addr().unwrap().ip(), new_user.public_key, new_user.name);
//     return new_user;
// }
