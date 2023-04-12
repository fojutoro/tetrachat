use std::net::{TcpListener};
use std::sync::{Arc};
use std::thread;

#[path = "./server-modules/database.rs"]
mod database;

#[path = "server-modules/requests.rs"]
mod requests;

fn main() {
    let user_list = Arc::new(requests::UserList::new());
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let user_list_clone = user_list.clone();
                thread::spawn(move || {
                    // Handle the client request
                    requests::handle_client(stream, user_list_clone);
                });
            }
            Err(e) => {
                println!("{} 2", e);
            }
        }
    }

    drop(listener);
}