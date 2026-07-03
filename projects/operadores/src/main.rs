fn main() {
    // Operadores servem para fazermos contas no computador, veja:

    let x = 10;
    let y = 10;

    println!("A soma dos numeros é: {}", x+y);

    println!("{}", x-y);
    println!("{}", x*y);
    println!("{}", x/y);
    println!("{}", x%y);

    let mut a = 100;
    let mut b = 9;

    // Faz a operação primeiro, depois imprime o resultado de 'a'
    a -= b;
    println!("{}", a); // Imprime 91

    a += b;
    println!("{}", a); // Volta para 100

    a *= b;
    println!("{}", a); // Imprime 900

    a /= b;
    println!("{}", a); // Imprime 100 de novo
}