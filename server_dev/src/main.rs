pub mod server;
use crate::server::server_comm::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    server_run(&listener);
    // handle_response(&listener);
}


//PoC = use rust to communicate and operate a server. Server asks for the user for a username and password to access the server. User inputs from terminal. Server sends a
//signal back if the correct combination is used.
