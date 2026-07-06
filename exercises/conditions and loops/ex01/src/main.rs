// Exercício 1 (Fácil)
// Desenvolva um programa que declare uma variável inteira contendo um número qualquer.
// Utilize uma estrutura if-else para verificar se esse número é positivo, negativo ou
// igual a zero. Ao final, exiba uma mensagem indicando a classificação do número.

fn main() {
    let number:i32 = 10;

    if number  == 0 {
        println!("O número inserido é 0!");
    } else if number < 0{
        println!("O número inserido é negativo.");
    } else {
        println!("O número inserido é positivo.");
    }
}
