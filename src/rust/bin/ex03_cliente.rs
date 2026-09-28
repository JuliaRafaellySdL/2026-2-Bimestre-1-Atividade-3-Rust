use std::io::Write;
use std::net::TcpStream;

fn main() {
    // Conecta ao servidor
    let mut conexao = TcpStream::connect("host.docker.internal:8080").unwrap();

    let mensagem = "Olá do outro computador!\n";

    // Envia a mensagem
    conexao.write_all(mensagem.as_bytes()).unwrap();

    println!("Mensagem enviada!");
}
