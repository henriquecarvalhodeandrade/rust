// Exercício 2: Somatório de Intervalo Definido
// Desenvolva um programa que calcule e exiba a soma de todos os números inteiros e positivos que estão localizados no intervalo fechado de 1 a 50 (ou seja, incluindo o 1 e o 50). Utilize um laço do tipo for para iterar sobre o intervalo matemático e acumular o valor em uma variável mutável, exibindo o resultado final apenas uma vez após o encerramento do laço.

fn soma_intervalo() {
    let mut soma: u32 = 0;

    for i in 1..=50 {
        
        println!("Passo {}: O acumulador está em {}, somando com o novo número {}", i, soma, i);
        
        soma += i;
    }

    println!("A soma de todos os números de 1 a 50 é: {}", soma);
}

fn main() {
    soma_intervalo();
}