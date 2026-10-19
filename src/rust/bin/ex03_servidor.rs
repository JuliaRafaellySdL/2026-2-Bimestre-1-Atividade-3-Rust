use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() {
    // Cria um socket TCP e escuta conexões na porta 8080.
    let servidor = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Servidor esperando conexão...");

    // Aceita uma conexão TCP de um cliente.
    let (conexao, _) = servidor.accept().unwrap();
    let mut leitor = BufReader::new(conexao);

    let mut mensagem = String::new();

    // Recebe a mensagem
    leitor.read_line(&mut mensagem).unwrap();

    println!("Mensagem recebida: {}", mensagem.trim_end());
}
