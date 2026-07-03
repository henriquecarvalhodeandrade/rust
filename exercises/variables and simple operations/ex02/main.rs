// 2. Antecessor e sucessor: Desenvolva um programa que declare uma variável inteira contendo um número qualquer. A partir desse valor, calcule e exiba seu antecessor e seu sucessor. Organize o código com comentários para deixar claro o que está sendo calculado.

fn antecessor_e_sucessor(x:i32){
    println!("O valor inserido é: {}, \nseu antecessor é: {} e \nseu sucessor é: {}\n",x ,x-1, x+1);
}

fn main() {
    let numero:i32 = 10;
    antecessor_e_sucessor(numero);
}