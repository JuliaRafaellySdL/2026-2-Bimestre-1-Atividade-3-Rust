use std::fs::File;
use std::io::Write;

fn main() {
    let mut arquivo = File::create("mensagem.txt").unwrap();

    arquivo
        .write_all("Olá do outro processo!".as_bytes()) //torna o "á" aceitavel para o compilador
        .unwrap();

    println!("Mensagem enviada!");
}
