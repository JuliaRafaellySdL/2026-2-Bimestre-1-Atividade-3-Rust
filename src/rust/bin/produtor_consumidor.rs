use comunicacao_entre_tarefas::produzir_dados;
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("iniciou");
    let (tx, rx) = mpsc::channel();
    // tx --> transmissor
    // rx --> receptor

    // Thread 1 - Produz os dados
    let thread_produtor = thread::spawn(move || {
        // move --> tranfere os valores para a tx
        println!("# produzir - iniciado");
        let dados = produzir_dados();
        println!("# produzir {dados:?}");
        // :? --> formato de depuração
        tx.send(dados).expect("não foi possível enviar os dados");
        // send --> manda o vetor para o canal
        // expect --> caso não tenha receptor disponível,
        // encerra o programa
        println!("# produzir - terminado");
    });

    // Thread 2 - Recebe e soma os dados
    let thread_consumidor = thread::spawn(move || {
        // move --> tranfere os valores para a rx
        println!("### consumir - iniciado");
        let dados = rx.recv().expect("não foi possível receber os dados");
        // recv() --> bloqueia a thread enquanto não tiver dados
        println!("### dados -> {dados:?}");
        let resultado: u32 = dados.iter().sum();
        println!("### resultado -> {resultado}");
        println!("### consumir - terminado");
    });

    thread_produtor.join().expect("a thread produtora falhou");
    // Espera a thread produtor terminar ou falhar
    thread_consumidor.join().expect("a thread consumidora falhou");
    // Espera a thread consumidor terminar ou falhar
    println!("finalizou");
}
