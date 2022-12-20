use std::{
    collections::HashMap,
    io::ErrorKind,
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

mod lib;

use lib::{request::Request, response::Response, tcpstreamreader::TcpStreamReader};

enum LogStatus<'a> {
    Request(&'a Request),
    GetConnection,
}

fn print_log(ip: IpAddr, log: LogStatus, storage_size: usize) {
    print!("{} [{}] ", ip, chrono::Utc::now());
    match log {
        LogStatus::Request(request) => match request {
            Request::Store { key, hash } => {
                print!(
                    "Received request to write new value {} by key {}. ",
                    hash, key
                )
            }
            Request::Load { key } => {
                print!("Received request to get value by key {}. ", key)
            }
        },
        LogStatus::GetConnection => {
            print!("Connection established. ");
        }
    }
    println!("Storage size: {}.", storage_size)
}

fn handle_connection(stream: TcpStream, storage: Arc<Mutex<HashMap<String, String>>>) {
    let ip = stream.peer_addr().unwrap().ip();
    print_log(ip, LogStatus::GetConnection, storage.lock().unwrap().len());
    let mut reader = TcpStreamReader::new(stream);
    thread::spawn(move || loop {
        let request = match reader.receieve_request() {
            Ok(x) => x,
            Err(error)
                if error.kind() == ErrorKind::ConnectionAborted
                    || error.kind() == ErrorKind::ConnectionReset =>
            {
                break
            }
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => continue,
            Err(_) => break,
        };
        print_log(
            ip,
            LogStatus::Request(&request),
            storage.lock().unwrap().len(),
        );
        reader
            .send_response(match request {
                Request::Store { key, hash } => {
                    storage.lock().unwrap().insert(key, hash);
                    Response::SuccessStore
                }
                Request::Load { key } => match storage.lock().unwrap().get(&key) {
                    Some(hash) => Response::SuccessLoad {
                        key,
                        hash: hash.clone(),
                    },
                    None => Response::KeyNotFound,
                },
            })
            .unwrap();
    });
}

fn main() {
    let listener = TcpListener::bind("localhost:6969").unwrap();
    let main_storage = HashMap::new();
    let reference = Arc::new(Mutex::new(main_storage));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let storage = reference.clone();
        handle_connection(stream, storage);
    }
}
