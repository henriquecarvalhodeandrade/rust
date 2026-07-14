// Exercício 4: Validador de Sequências de Collatz
// A Conjectura de Collatz afirma que se você pegar qualquer número inteiro positivo e aplicar duas regras simples (se o número for par, divida-o por 2; se for ímpar, multiplique-o por 3 e adicione 1), a sequência eventualmente sempre atingirá o número 1. Crie um programa em Rust que solicite um número inteiro inicial ao usuário através do teclado e utilize um laço while para calcular e exibir cada termo dessa sequência na tela linha por linha. O loop deve continuar rodando e aplicando as regras matemáticas de forma dinâmica até que o valor final estabilize exatamente em 1, imprimindo também o número total de passos necessários para chegar ao fim.

use std::io;

fn conjectura_collatz(mut numero:u32) {

    let mut passos = 0;

    while numero != 1 {
        if (numero % 2) == 0 {
            numero /= 2;
        } else {
            numero = (numero * 3) + 1;
        }
        passos += 1;

        println!("-> {}", numero);
    }

    println!("Operaçao finalizada! Foram necessarios {} passos para tal.", passos);

}

fn main() {
    
    loop {
        
        println!("Insira um número inteiro e positivo.");
        let mut entrada = String::new();

        io::stdin()
            .read_line(&mut entrada)
            .expect("Falha ao ler linha.");
        //

        let numero = match entrada.trim().parse() {
            Ok(num) => num,     // Se a conversão der certo, retorna o número.
            Err(_) => continue, // Se falhar (ex: digitou letras), o '_' ignora o erro e reinicia o loop.
        };

        if numero == 0 {
            println!("[ERRO] O número deve ser maior que zero!\n");
            continue;
        }
    
        conjectura_collatz(numero);
        break;
    }

    

}
