use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::{HashMap, VecDeque};
type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;
type History = Arc<Mutex<VecDeque<String>>>;
fn handle_client(mut stream: TcpStream, clients: Clients, history: History) {
    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).unwrap();
    let name = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    let name = name.trim().to_string();
    {
        let mut clients_guard = clients.lock().unwrap();
        if clients_guard.contains_key(&name) {
            stream.write_all(b"Nome j\xE1 est\xE1 em uso, escolha outro nome.\n").unwrap();
            return;
        } else {
            clients_guard.insert(name.clone(), stream.try_clone().unwrap());
        }
    }
    println!("{} entrou no chat", name);
    broadcast_message(&clients, &format!("{} entrou no chat", name), &name);
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("{} saiu do chat", name);
                clients.lock().unwrap().remove(&name);
                return;
            },
            Ok(n) => n,
            Err(_) => {
                eprintln!("Erro ao ler do cliente");
                return;
            }
        };
        let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
        let message = message.trim().to_string();
        if message == "/list" {
            let clients_guard = clients.lock().unwrap();
            let user_list = get_user_list(&clients_guard);
            stream.write_all(format!("Usuários online: {}\n", user_list).as_bytes()).unwrap();
            continue;
        }
        println!("{}: {}", name, message);
        if message.starts_with("/history") {
            let history = get_history(&history);
            if let Err(_) = stream.write_all(history.as_bytes()) {
                eprintln!("Erro ao enviar histórico para o cliente");
            }
            continue;
        } else {
            let mut history_guard = history.lock().unwrap();
            history_guard.push_back(format!("{}: {}", name, message));
        }
        let mut clients_guard = clients.lock().unwrap();
        for (client_name, client_stream) in clients_guard.iter_mut() {
            if client_name != &name {
                client_stream.write_all(format!("{}: {}\n", name, message).as_bytes()).unwrap();
            }
        }
    }
}
fn get_history(history: &History) -> String {
    let history_guard = history.lock().unwrap();
    history_guard.iter().cloned().collect::<Vec<String>>().join("\n")
}
fn broadcast_message(clients: &Clients, message: &str, sender_name: &str) {
    let clients_guard = clients.lock().unwrap();
    for (client_name, mut client_stream) in clients_guard.iter() {
        if client_name != sender_name {
            client_stream.write_all(format!("{}", message).as_bytes()).unwrap();
        }
    }
}
fn get_user_list(clients: &HashMap<String, TcpStream>) -> String {
    clients.keys().cloned().collect::<Vec<String>>().join(", ")
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let history: History = Arc::new(Mutex::new(VecDeque::new()));
    println!("Servidor rodando em 127.0.0.1:8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let clients = Arc::clone(&clients);
                let history = Arc::clone(&history);
                thread::spawn(move || {
                    handle_client(stream, clients, history);
                });
            }
            Err(e) => {
                eprintln!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
}
