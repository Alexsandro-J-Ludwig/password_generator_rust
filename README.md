# Gerador de Senhas Básico com Rust
O objetivo desse projeto simples é treinar meu entendimento sobre o uso de bibliotecas externas e manipulação de variáveis, semelhante a um projeto antigo em JavaScript.

## Como funciona
O gerador de senhas funciona utilizando a biblioteca [rand](https://docs.rs/rand/latest/rand/index.html) e suas funções com rng, definindo um comprimento aleatório de 8 a 50 caracteres e permitindo que o usuário selecione o que a senha vai conter (letras maiúsculas e minúsculas, números e símbolos).

## Dependências
rand 0.8

## Como posso testar?
Diferente do meu código anterior [vetor-manager-rust](https://github.com/Alexsandro-J-Ludwig?tab=repositories), este não pode ser testado no [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024) por conter uma dependência externa necessitando do arquivo ```Cargo.toml```.

1. Faça uma cópia do repositório usando o comando: ```git clone https://github.com/Alexsandro-J-Ludwig/password_generator_rust```.
2. Acesse o caminho do seu arquivo pelo terminal de comando usando o comando: ```cd <Caminho do repositório>```.
3. Digite o seguinte comando: ```cargo run```.
