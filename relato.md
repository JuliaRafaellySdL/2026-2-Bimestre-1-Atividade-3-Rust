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
> Troca de dados; <br>
> Coordenação da execução; <br>
> Não concorrente.
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
> texto explicando o código
<br>
> mostrar o código completo

#### EXECUÇÃO
> explicar como foi executado
> mostrar as saídas do terminal
> mostrar as saídas do terminal

##### PROBLEMAS
> se houve problema na execução, enumerar os problemas e suas respectivas soluções

### Comunicação entre tarefas em processos diferentes no mesmo computador

#### CÓDIGO
> texto explicando o código
> mostrar o código completo

#### EXECUÇÃO
> explicar como foi executado
> mostrar as saídas do terminal
> mostrar as saídas do terminal

#### PROBLEMAS
> se houve problema na execução, enumerar os problemas e suas respectivas soluções

### Comunicação entre tarefas em processos diferentes em computadores diferentes

#### CÓDIGO
> texto explicando o código
> mostrar o código completo

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
