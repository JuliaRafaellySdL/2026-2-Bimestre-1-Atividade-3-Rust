use std::io::Write;
use std::net::TcpStream;

fn main() {
    // Usa o endereço informado; por padrão, conecta ao servidor local.
    let endereco = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let mut conexao = TcpStream::connect(&endereco)
        .unwrap_or_else(|erro| panic!("não foi possível conectar a {endereco}: {erro}"));

    let mensagem = "Olá do outro computador!\n";

    // Envia a mensagem
    conexao.write_all(mensagem.as_bytes()).unwrap();

    println!("Mensagem enviada!");
}

//docker-compose.yml
//  Lista os computadores/máquinas
//  A parte de versionamento deve ser omitida
//  Docker compose -> containers como serviço
//      Serviço cliente e serviço servidor
//      yaml -> estrutura
