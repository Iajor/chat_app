use std::process::Command;
use std::net::TcpStream;
use std::io::Write;
use std::thread;
use std::time::Duration;

fn start_server() -> std::process::Child {
    Command::new("cargo")
        .args(&["run", "--bin", "server"])
        .spawn()
        .expect("Falha ao iniciar o servidor")
}

fn stop_server(server_process: &mut std::process::Child) {
    server_process.kill()
        .expect("Falha ao finalizar o processo do servidor");
}
#[test]
fn test_start_server(){
    let mut server = start_server();
    thread::sleep(Duration::from_secs(2));

    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Falha ao conectar-se ao servidor");

    stream.write_all(b"Teste").expect("Falha ao escrever para o servidor");
    thread::sleep(Duration::from_secs(2));

    drop(stream);
    stop_server(&mut server);
}