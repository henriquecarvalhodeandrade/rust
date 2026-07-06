// Exercício 3 (Médio)
// Desenvolva um programa que declare duas variáveis inteiras e uma terceira variável
// contendo um caractere que representa uma operação matemática ('+', '-', '*' ou '/').
// Utilize a estrutura match para identificar qual operação deve ser realizada entre os
// dois números e exiba o resultado. Caso seja informado um operador inválido, exiba
// uma mensagem indicando que a operação não é reconhecida.

fn qual_operacao(operador: &str, num1: i32, num2: i32) {
    match operador {
        "+" => println!("Resultado: {}", num1 + num2),

        "-" => println!("Resultado: {}", num1 - num2),

        "*" => println!("Resultado: {}", num1 * num2),

        "/" => println!("Resultado: {}", num1 / num2),

        "%" => println!("Resultado: {}", num1 % num2),

        "**" => println!("Resultado: {}", num1.pow(num2 as u32)),

        _ => println!("Operador inválido."),
    }
}

fn main() {
    let operador = "+";
    let num1 = 10;
    let num2 = 15;


    qual_operacao(operador, num1, num2);
}
