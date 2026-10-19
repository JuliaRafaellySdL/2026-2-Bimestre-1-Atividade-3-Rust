use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut mensagem = String::new();

    // Lê uma linha enviada pelo primeiro processo pelo pipe.
    stdin
        .lock()
        .read_line(&mut mensagem)
        .expect("não foi possível receber a mensagem pelo pipe");

    println!("entrada: {}", mensagem.trim_end());
}
