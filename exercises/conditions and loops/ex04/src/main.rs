// Exercício 4 (Difícil)
// Desenvolva um programa que declare uma variável representando o valor total de uma
// compra. Utilizando uma cadeia de if-else if, aplique o desconto correspondente de
// acordo com as seguintes regras: compras de até R$100 não recebem desconto; compras
// acima de R$100 até R$500 recebem 10% de desconto; compras acima de R$500 até
// R$1000 recebem 15% de desconto; e compras acima de R$1000 recebem 20% de desconto.
// Ao final, exiba o valor original da compra, o percentual de desconto aplicado e o
// valor final a ser pago.

fn calcular_desconto(carrinho:f32){

    if carrinho <= 0.0 {
        println!("Valor informado é inválido.");

    } else if carrinho <= 100.0 {
        println!("Suas compras, de valor {}, não receberam desconto.\n", carrinho);

    } else if carrinho <= 500.0 {
        println!("Suas compras, do valor de {}, receberam desconto de 10%, valor final: {}", carrinho, carrinho - (carrinho * (10.0 / 100.00)));

    } else if carrinho <= 1000.0 {
        println!("Suas compras, do valor de {}, receberam desconto de 15%, valor final: {}", carrinho, carrinho - (carrinho * (15.0 / 100.00)));

    } else {
        println!("Suas compras, do valor de {}, receberam desconto de 20%, valor final: {}", carrinho, carrinho - (carrinho * (20.0 / 100.00)));
    }
    

}

fn main() {
    
    let carrinho:f32 = 900.0;
    calcular_desconto(carrinho);

}
