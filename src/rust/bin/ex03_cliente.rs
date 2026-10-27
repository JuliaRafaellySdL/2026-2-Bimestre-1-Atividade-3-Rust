use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    // No Docker Compose, o nome do serviço consumidor funciona como endereço.
    let endereco = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Aguarda o consumidor ficar pronto antes de abandonar a conexão.
    let mut tentativas = 0;
    let mut conexao = loop {
        match TcpStream::connect(&endereco) {
            Ok(conexao) => break conexao,
            Err(_) if tentativas < 20 => {
                tentativas += 1;
                thread::sleep(Duration::from_millis(250));
            }
            Err(erro) => panic!("não foi possível conectar a {endereco}: {erro}"),
        }
    };

    let dados = [10, 20, 30];
    let mensagem = dados
        .iter()
        .map(|valor| valor.to_string())
        .collect::<Vec<_>>()
        .join(",");

    // Envia os dados pelo socket TCP, terminando a mensagem com uma quebra de linha.
    conexao
        .write_all(format!("{mensagem}\n").as_bytes())
        .expect("não foi possível enviar os dados");

    println!("Dados enviados: {mensagem}");
}
