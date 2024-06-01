use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor}};
use crossterm::terminal::{Clear, ClearType};
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Não foi possível se conectar ao servidor");
    let mut name = String::new();
    println!("Digite seu nome (código): ");
    io::stdin().read_line(&mut name).expect("Falha ao ler nome");
    let name = name.trim().to_string();
    stream.write_all(name.as_bytes()).expect("Falha ao enviar nome");
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        ResetColor
    ).expect("Erro ao configurar cores");
    let user_colors = Arc::new(Mutex::new(HashMap::new()));
    let mut stream_write = stream.try_clone().expect("Falha ao clonar stream");
    let name_clone = name.clone();
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut buffer = String::new();
        loop {
            buffer.clear();
            stdin.read_line(&mut buffer).expect("Falha ao ler entrada");
            let message = buffer.trim().to_string();
            if message == "/quit" {
                println!("Desconectando...");
                stream_write.shutdown(std::net::Shutdown::Both).expect("Falha ao desconectar");
                break;
            }
            print!("\x1B[1A\x1B[2K");
            io::stdout().flush().unwrap();
            let formatted_message = format!("{}\n", message);
            stream_write.write_all(formatted_message.as_bytes()).expect("Falha ao enviar mensagem");
            if message != "/list" {
                print_colored_message("Você", &message, hash_str_to_color(&name_clone));
            }
            if message.trim() == "/help" {
                print_help();
            }
        }
    });
    println!("{} entrou no chat", name);
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Conexão fechada pelo servidor");
                break;
            }
            Ok(n) => {
                let received_message = String::from_utf8_lossy(&buffer[..n]);
                if received_message.starts_with("Usuários online:") {
                    println!("{}", received_message);
                    continue;
                }
                let parts: Vec<&str> = received_message.splitn(2, ':').collect();
                if parts.len() == 2 {
                    let (sender, message) = (parts[0].trim(), parts[1].trim());
                    let mut user_colors = user_colors.lock().unwrap();
                    let color = user_colors
                        .entry(sender.to_string())
                        .or_insert_with(|| hash_str_to_color(sender));
                    if sender != name {
                        print_colored_message(sender, message, *color);
                    }
                } else {
                    println!("{}", received_message);
                }
            }
            Err(_) => {
                eprintln!("Erro ao ler do servidor");
                break;
            }
        }
    }
}
fn print_colored_message(sender: &str, message: &str, color: Color) {
    execute!(
        io::stdout(),
        SetForegroundColor(color),
        Print(format!("{}: {}\n", sender, message)),
        ResetColor
    ).expect("Erro ao configurar cores");
}
fn hash_str_to_color(s: &str) -> Color {
    let mut hash: u64 = 0;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    let r = ((hash >> 16) & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = (hash & 0xFF) as u8;
    Color::Rgb { r, g, b }
}
fn print_help() {
    println!("Comandos disponíveis:");
    println!("/help - Mostra a lista de comandos disponíveis e suas descrições");
    println!("/history - Retorna o histórico de mensagens");
    println!("/list - Lista todos os usuários online no chat");
    println!("/quit - Desconecta do chat");
}