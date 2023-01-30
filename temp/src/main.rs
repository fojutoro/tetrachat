#[path = "client-modules/client_requests.rs"] mod client_requests;
#[path = "client-modules/utils.rs"] mod utils;

fn main() {
    client_requests::menu();
}