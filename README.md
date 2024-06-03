# ChatApp

Este é um aplicativo de chat simples escrito em Rust. Ele permite que vários clientes se conectem a um servidor e se comuniquem em tempo real.

## Objetivo e Características

O objetivo principal do ChatApp é servir como uma ferramenta de estudo para praticar Rust e conceitos de desenvolvimento web, sem a complexidade de integração com banco de dados. Também, não fora dada muita atenção a uma interface moderna, sendo escolhida a nostalgia de um chat secreto de filmes antigos.

- **Mensagens em Tempo Real:** Clientes podem enviar e receber mensagens em tempo real.
- **Listagem de Usuários:** Clientes podem ver a lista de usuários online.
- **Histórico de Mensagens:** Clientes podem solicitar o histórico de mensagens do servidor.
- **Saída Colorida:** As aplicações do servidor e do cliente utilizam saída colorida para melhor legibilidade.

## Tecnologias e Práticas de Estudo

- **Rust:** O aplicativo é escrito em Rust, o que proporcionou uma ótima oportunidade para praticar essa linguagem de programação moderna e segura. A IDE utilizada fora [RustRover](https://www.jetbrains.com/pt-br/rust/) da JetBrains.
- **Desenvolvimento Web:** Para construir o servidor de chat, foram aplicados conhecimentos de desenvolvimento web, incluindo o uso de sockets para comunicação em tempo real.
- **Programação Concorrente:** Rust oferece ótimas ferramentas para lidar com programação concorrente de forma segura, e esses conceitos foram aplicados no desenvolvimento do servidor para lidar com múltiplos clientes simultaneamente.

## Instalação

Para usar o ChatApp, você precisa ter o Rust instalado em seu sistema. Você pode instalar o Rust a partir [daqui](https://www.rust-lang.org/tools/install).

Depois de instalar o Rust, você pode clonar este repositório e compilar o projeto usando o Cargo:

```bash
https://github.com/Iajor/chat_app.git
cd chat_app
cargo build --release
```

## Uso

1. Inicie o servidor executando:

```bash
cargo run --bin server
```

2. Os clientes podem se conectar ao servidor usando:

```bash
cargo run --bin client
```

Siga as instruções para inserir seu nome e começar a conversar!

## Comandos

- `/list`: Lista todos os usuários online.
- `/history`: Recupera o histórico de mensagens.
- `/help`: Informa os comandos disponíveis.
- `/quit`: Desconecta do chat.

Sinta-se à vontade para adicionar mais comandos.

## Pseudocódigo

Rust é uma linguagem difícil, porém, depois que se compreendem os conceitos e a sintaxe, as peças se encaixam facilmente. Aqui está um pseudocódigo simplificado que demonstra como o servidor e o cliente interagem:

### Servidor:
```
1. Inicialize uma conexão TCP com o servidor na porta 8080.
let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Não foi possível se conectar ao servidor");

2. Solicite ao usuário que insira seu nome.
println!("Digite seu nome (código): ");
io::stdin().read_line(&mut name).expect("Falha ao ler nome");

3. Envie o nome do usuário para o servidor.
stream.write_all(name.as_bytes()).expect("Falha ao enviar nome");

4. Limpe a tela.
execute!(
    io::stdout(),
    Clear(ClearType::All),
    ResetColor
).expect("Erro ao configurar cores");

5. Crie um hashmap para armazenar as cores dos usuários.
let user_colors = Arc::new(Mutex::new(HashMap::new()));

6. Clone o stream para escrita para uso em uma thread separada.
let mut stream_write = stream.try_clone().expect("Falha ao clonar stream");

7. Inicialize uma thread para receber mensagens do servidor.
thread::spawn(move || {

8. Em uma thread separada:
   loop {
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

9. Exiba uma mensagem indicando que o usuário entrou no chat.
println!("{} entrou no chat", name);

10. Enquanto a conexão estiver ativa:
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

11. Encerre o programa se a conexão for fechada ou ocorrer um erro.
```

### Cliente:
```
1. Inicialize uma conexão TCP com o servidor na porta 8080.
    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Não foi possível se conectar ao servidor");

2. Solicite ao usuário que insira seu nome.
    println!("Digite seu nome (código): ");
    io::stdin().read_line(&mut name).expect("Falha ao ler nome");

3. Envie o nome do usuário para o servidor.
    stream.write_all(name.as_bytes()).expect("Falha ao enviar nome");

4. Limpe a tela.
    execute!(
        io::stdout(),
        Clear(ClearType::All),
        ResetColor
    ).expect("Erro ao configurar cores");

5. Crie um hashmap para armazenar as cores dos usuários.
    let user_colors = Arc::new(Mutex::new(HashMap::new()));

6. Clone o stream para escrita para uso em uma thread separada.
    let mut stream_write = stream.try_clone().expect("Falha ao clonar stream");

7. Inicialize uma thread para receber mensagens do servidor.
    thread::spawn(move || {

8. Em uma thread separada:
       loop {
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

9. Exiba uma mensagem indicando que o usuário entrou no chat.
    println!("{} entrou no chat", name);

10. Enquanto a conexão estiver ativa:
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

11. Encerre o programa se a conexão for fechada ou ocorrer um erro.
```
## Licença

Este projeto está licenciado sob a Licença MIT - consulte o arquivo [LICENSE](LICENSE) para obter detalhes.
