use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() {
    // Cria um socket TCP e escuta conexões na porta 8080.
    let servidor = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Servidor esperando conexão...");

    // Aceita a conexão TCP do produtor.
    let (conexao, _) = servidor.accept().unwrap();
    let mut leitor = BufReader::new(conexao);

    let mut mensagem = String::new();

    // Recebe os dados enviados pelo produtor.
    leitor.read_line(&mut mensagem).unwrap();

    let dados: Vec<u32> = mensagem
        .trim()
        .split(',')
        .map(|valor| valor.parse().expect("dado recebido não é um número válido"))
        .collect();
    let soma: u32 = dados.iter().sum();

    println!("Dados recebidos: {dados:?}");
    println!("Soma calculada: {soma}");
}
