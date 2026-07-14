// Exercício 1: Contador Regressivo Estrito
// Escreva um programa em Rust que utilize uma estrutura de repetição para exibir uma contagem regressiva de 10 até 1 na tela. Logo após o término do loop, o programa deve exibir a mensagem "Fogo!". Tente resolver este exercício utilizando o laço condicional while e, em seguida, refatore o código utilizando um laço for com um iterador de intervalo reverso (.rev()).

fn laco_while(){
    let mut n = 10; 

    while n != 0 {
        println!("{}", n);
        n -= 1;
    }

    println!("Fogo!");
}


fn laco_for(){
    
    // o metodo rev() inverte a ordem, logo, ao inves de contagem em ordem crescente, ele contara em ordem decrescente. Na verdade é assim que ele funciona: ao inves de fazer de 1 ate 11, ele faz de 11 até 1. Ou seja ele inverte a ordem dos parametros, tendeu?
    for n in (1..11).rev() {
        println!("{}", n);
    }

    // Outra alternativa para o range do for seria:
    // for n in (1..=10).rev() {
    //     println!("{}", n);
    // }

    println!("Fogo!");
}


fn main() {
    
    laco_while();
    laco_for();
}
