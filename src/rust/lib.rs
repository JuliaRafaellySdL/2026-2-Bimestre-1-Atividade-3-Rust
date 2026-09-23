// Função compartilhada
use rand::Rng; // Importa a trait que permite números aleatórios

pub fn produzir_dados() -> Vec<u32> {
    // Vec<u32> devolve um vetor de "unsigned int" de 32 bits
    // pub --> permite que outro arquivos usem
    let mut rng = rand::thread_rng();
    // mut --> permite que seja alterado
    (0..100)
        .map(|_| rng.gen_range(0..=110))
        .collect()
    // 0..100 --> sequência de 100 posições
    // .map() --> cada posição em um u32
    // |_| --> posição não será usada
    // gen_range() --> sorteia um número de 0 a 110
    // .collect() --> reúne os números
}
