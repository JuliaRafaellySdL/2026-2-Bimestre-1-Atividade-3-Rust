use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() {
    // Abre a porta 8080
    let servidor = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Servidor esperando conexão...");

    // Espera um computador se conectar
    let (conexao, _) = servidor.accept().unwrap();
    let mut leitor = BufReader::new(conexao);

    let mut mensagem = String::new();

    // Recebe a mensagem
    leitor.read_line(&mut mensagem).unwrap();

    println!("Mensagem recebida: {}", mensagem.trim_end());
}
