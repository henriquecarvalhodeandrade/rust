fn somatorio(x: i32, y: i32) {
    // A função somatorio tem como objetivo somar os dois parâmetros
    // e exibir o resultado da soma.
    println!("Resultado da soma do primeiro número com o segundo: {}", x + y);
}

fn main() {
    // Variáveis que armazenam os valores
    let numero1: i32 = 10;
    let numero2: i32 = 20;

    // Chamando a função e passando os valores
    somatorio(numero1, numero2);
}