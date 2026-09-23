use comunicacao_entre_tarefas::produzir_dados;
// Importa a função

fn consumir_dados(dados: &[u32]) {
    // &[u32] --> "slice" de números
    let resultado: u32 = dados.iter().sum();
    // .iter() --> percorre os números
    // .sum() --> soma seus valores
    println!("recebeu -> {resultado}");
}

fn main() {
    println!("iniciou");
    let dados = produzir_dados();
    consumir_dados(&dados);
    println!("finalizou");
}
