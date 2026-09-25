se std::sync::mpsc;
use std::thread;

fn main() {
    // Cria um canal
    let (tx, rx) = mpsc::channel();

    // Cria uma nova thread
    thread::spawn(move || {
        let mensagem = "Olá da outra thread!";

        // Envia a mensagem
        tx.send(mensagem).unwrap();
    });

    // Recebe a mensagem
    let mensagem_recebida = rx.recv().unwrap();

    println!("Mensagem recebida: {}", mensagem_recebida);
}