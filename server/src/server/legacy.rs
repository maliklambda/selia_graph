use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use crate::utils::constants::server::{CLOSE_CONNECTION_MSG, DEFAULT_PORT, get_host_name_full};

pub type ConnectionId = u64;
pub type ResponseSenderId = u64;

pub fn init_server() {
    println!("Initializing server on port {}", DEFAULT_PORT);
    let mut active_conns: HashMap<ConnectionId, Arc<Mutex<TcpStream>>> = HashMap::new();
    let listener =
        TcpListener::bind(get_host_name_full()).expect("Failed to bind listener to port {PORT}");

    let mut last_id: u64 = 0;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream = Arc::new(Mutex::new(stream));
                let id = last_id;
                last_id += 1;
                acknowledge_conn(stream.clone());
                authenticate_conn(stream.clone());
                active_conns.insert(id, stream.clone());
                std::thread::spawn(move || handle_client(id, stream));
                println!("new connection. Active connections: {:?}", active_conns);
            }
            Err(err) => {
                println!("Error accepting request: {err}");
            }
        }
    }
}

fn handle_client(id: u64, stream: Arc<Mutex<TcpStream>>) {
    let mut buf = [0_u8; 512];
    loop {
        let mut s = stream.lock().unwrap();
        match s.read(&mut buf) {
            Ok(n) => {
                assert!(n <= 512);
                if &buf[..n] == CLOSE_CONNECTION_MSG {
                    break;
                }
                let query_str = String::from_utf8_lossy(&buf[..n]);
                println!("Query = {}", query_str);
            }
            Err(err) => panic!("Error reading query: {err}"),
        }
    }
    // for _ in [1;3] {
    //     println!("Keeping conn #{id} alive");
    //     std::thread::sleep(timeout_dur);
    //     let mut s = stream.lock().unwrap();
    //     println!("s: {:?}", s);
    //     s.write_all(ack_msg).unwrap();
    //     s.flush().unwrap();
    // }

    println!("Closing connection #{id}");
    let mut s = stream.lock().unwrap();
    s.write_all(CLOSE_CONNECTION_MSG).unwrap();
    s.flush().unwrap();
}

fn acknowledge_conn(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0u8; 512];
    let mut stream = stream.lock().unwrap();
    match stream.read(&mut buffer) {
        Ok(n) => {
            // send ack
            println!("Received: '{}'", String::from_utf8_lossy(&buffer[..n]));
            let conn_id = "b5e4324d-e136-42a4-bcdf-6e97ade63ce3";
            let response = format!("Connection acknowledged: {conn_id}");
            stream
                .write_all(response.as_bytes())
                .expect("failed to send response");
            stream
                .flush()
                .expect("failed to flush buffer after sending response");
        }
        Err(err) => println!("Error reading into buffer: {err}"),
    }
}

fn authenticate_conn(stream: Arc<Mutex<TcpStream>>) {
    let mut buffer = [0u8; 512];
    let mut stream = stream.lock().unwrap();
    match stream.read(&mut buffer) {
        Ok(n) => {
            let s = String::from_utf8_lossy(&buffer[..n]);

            // read client_ack & parse credentials
            println!("Received: '{}'", s);

            let (username, password) = {
                let i_username = s
                    .find("user=")
                    .expect("No user credentials found. Expected 'user='");
                let i_password = s
                    .find("password=")
                    .expect("No user credentials found. Expected 'password='");
                let username = &s[i_username..i_password];
                let password = &s[i_password..];
                (username, password)
            };

            println!("Authenticating user '{username}' with password '{password}'");
        }
        Err(err) => println!("Error reading into buffer: {err}"),
    }
}
