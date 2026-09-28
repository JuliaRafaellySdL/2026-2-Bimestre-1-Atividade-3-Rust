use std::fs;

fn main() {
    let mensagem = fs::read_to_string("mensagem.txt").unwrap();

    println!("Mensagem recebida: {}", mensagem);
}
