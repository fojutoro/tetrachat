use std::net::{TcpListener};
use std::thread;

#[path = "mods/database.rs"] mod database;
#[path = "mods/requests.rs"] mod requests;

fn main() {
    // let mut users: Dict<User> = Dict::<User>::new();
    let listener = TcpListener::bind("localhost:8080").unwrap();
    for stream in listener.incoming() {
        // When a client joins for the first time he has to send an ACR - Account creation request. Then we create an account for him.
        match stream {
            Ok(stream) => {
                println!(
                    "local socket address: {}",
                    stream.local_addr().unwrap().ip(),
                );
                thread::spawn(move||{
                    requests::handle_client(stream)
                });
            }
            Err(e) => {
                println!("{} 2", e);
            }
        }
    }
    drop(listener);
}