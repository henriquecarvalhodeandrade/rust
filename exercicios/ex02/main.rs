// 2. Antecessor e sucessor: Desenvolva um programa que declare uma variável inteira contendo um número qualquer. A partir desse valor, calcule e exiba seu antecessor e seu sucessor. Organize o código com comentários para deixar claro o que está sendo calculado.

fn antecessor_e_sucessor(x:i32){
    println!("O valor é: {}, seu antecessor é: {} e seu sucessor é {}",x ,x-1, x+1);
}

fn main() {
    let numero:i32 = 10;
    antecessor_e_sucessor(numero);
}