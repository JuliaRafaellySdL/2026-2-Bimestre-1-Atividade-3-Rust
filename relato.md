# Relatório sobre implementação de comunicação entre tarefas em Rust

## Introdução

Este relato faz parte do processo avaliativo da disciplina de sistemas operacionas no curso superior em análise e desenvolvimento de sistemas, ofertado na Diretoria acadêmica de gestão e tecnologia da informação no campus natal-central do instituto federal de educação, ciência e tecnologia do rio grande do norte.

Tem como objetivo principal relatar as implementações de comunicação entre tarefas na linguagem Rust.

O grupo de trabalho foi formado por Julia Rafaelly Siqueira de Lima, Lídia Rebeka da Silva Fernandes e Lyonara da Silva Carmêlo.

## Comunicação entre tarefas em Rust

### Informações gerais

#### OBJETIVO DA COMUNICAÇÃO ENTRE TAREFAS:
Permitir que partes de um programa trabalhem juntas
<br>
> * Troca de dados; <br>
> * Coordenação da execução; <br>
> * Não concorrente.
<br>

Em Rust, essa troca acontece de maneira segura, sem as duas threads acessarem a mesma variável ao mesmo tempo de forma direta.

#### SOBRE O DOCKER:
O uso do docker se dá por praticidade, visto que não é necessária a instalação da linguagem Rust em diferentes máquinas. A configuração docker usada foi:
<br>
```dockerfile
FROM rust:1.85
WORKDIR /app
COPY Cargo.toml ./
COPY lib.rs ./lib.rs
COPY bin ./bin
RUN cargo build --release
ENTRYPOINT ["cargo", "run", "--release", "--bin"]
```
<br>

### Comunicação entre tarefas com linhas de execução no mesmo processo

#### CÓDIGO
O código cria duas threads capazes de se comunicar por meio de um __canal (channel)__
<br>

```rust
use std::sync::mpsc;
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
```

#### EXECUÇÃO
> explicar como foi executado
> mostrar as saídas do terminal
> mostrar as saídas do terminal

##### PROBLEMAS
> se houve problema na execução, enumerar os problemas e suas respectivas soluções

### Comunicação entre tarefas em processos diferentes no mesmo computador

#### CÓDIGO
> texto explicando o código

__Primeiro programa:__ Enviar mensagem
```rust
use std::fs::File;
use std::io::Write;

fn main() {
    let mut arquivo = File::create("mensagem.txt").unwrap();

    arquivo
        .write_all(b"Olá do outro processo!")
        .unwrap();

    println!("Mensagem enviada!");
}
```

__Segundo programa:__ Receber mensagem
```rust
use std::fs;

fn main() {
    let mensagem = fs::read_to_string("mensagem.txt").unwrap();

    println!("Mensagem recebida: {}", mensagem);
}
```

#### EXECUÇÃO
> explicar como foi executado
> mostrar as saídas do terminal
> mostrar as saídas do terminal

#### PROBLEMAS
> se houve problema na execução, enumerar os problemas e suas respectivas soluções

### Comunicação entre tarefas em processos diferentes em computadores diferentes

#### CÓDIGO
__Servidor:__
```rust
use std::io::Read;
use std::net::TcpListener;

fn main() {
    // Abre a porta 8080
    let servidor = TcpListener::bind("0.0.0.0:8080").unwrap();

    println!("Servidor esperando conexão...");

    // Espera um computador se conectar
    let (mut conexao, _) = servidor.accept().unwrap();

    let mut mensagem = String::new();

    // Recebe a mensagem
    conexao.read_to_string(&mut mensagem).unwrap();

    println!("Mensagem recebida: {}", mensagem);
}
```
__Cliente:__
```rust
use std::io::Write;
use std::net::TcpStream;

fn main() {
    // Conecta ao servidor
    let mut conexao = TcpStream::connect("192.168.0.10:8080").unwrap();

    let mensagem = "Olá do outro computador!";

    // Envia a mensagem
    conexao.write_all(mensagem.as_bytes()).unwrap();

    println!("Mensagem enviada!");
}
```


#### EXECUÇÃO
> explicar como foi executado
> mostrar as saídas do terminal
> mostrar as saídas do terminal

#### PROBLEMAS
> se houve problema na execução, enumerar os problemas e suas respectivas soluções

## Considerações finais

#### CONCLUSÃO
> conseguiu implementar tudo e executar?
> qual foi o aprendizado nesse trabalho?
> alguma recomendação para próximos alunos?
