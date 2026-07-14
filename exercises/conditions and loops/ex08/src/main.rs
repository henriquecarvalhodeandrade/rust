// Exercício 3: O Menu Interativo de Operações
// Construa um programa que simule um menu interativo de terminal utilizando a estrutura de repetição infinita loop. O programa deve exibir continuamente três opções para o usuário: digite 1 para exibir uma mensagem de boas-vindas, digite 2 para gerar e exibir um número aleatório entre 1 e 10 utilizando o crate rand, ou digite 3 para encerrar o programa. O sistema deve ler a entrada do teclado, converter o palpite para um inteiro de forma segura utilizando match, executar a ação correspondente e só quebrar o laço (break) quando a opção 3 for explicitamente escolhida.

use std::io;
use rand::Rng;

fn menu() {

    // inicio do programa
    println!("Digite seu nome de usuario.\n");

    let mut nome_usuario = String::new();

    io::stdin().read_line(&mut nome_usuario)
        .expect("Falha ao ler linha.");
    //

    // inicio do menu
    loop {

        println!("Digite a opçao do menu.\nOpçoes:\n1 - Boas vindas\n2 - Gerar um numero aleatorio dentre 1 a 10\n3 - Encerrar o programa\n");

        let mut opcao = String::new();

        io::stdin().read_line(&mut opcao)
            .expect("Falha ao ler linha.");
        // 

        match opcao.trim() {
            "1" => println!("Olá, Seja Bem vindo ao meu Programa, {}!\n", nome_usuario.trim()),
            "2" => {
                let numero_aleatorio = rand::thread_rng().gen_range(1, 11);
                println!("O numero gerado foi: {}", numero_aleatorio);
            },
            "3" => {
                println!("Encerrando o programa!\n");
                break;
            },
            _ => {
                println!("Erro, selecione uma opçao válida.");
                continue;
            }
        }
    }

}


fn main() {
    menu();
}
