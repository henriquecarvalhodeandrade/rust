// Exercício 5 (Difícil)
// Desenvolva um programa que declare três variáveis inteiras representando os lados
// de um possível triângulo. Antes de classificá-lo, verifique se os três valores
// realmente formam um triângulo utilizando a regra de que a soma de quaisquer dois
// lados deve ser maior que o terceiro. Caso não seja um triângulo, exiba uma mensagem
// informando essa condição. Caso seja válido, utilize estruturas condicionais para
// classificá-lo como Equilátero (três lados iguais), Isósceles (dois lados iguais)
// ou Escaleno (todos os lados diferentes).

fn classificar_triangulo(a:i32, b:i32, c:i32) {

    if a <= 0 || b <= 0 || c <= 0 {
        println!("Nenhum dos lados pode ter valor menor ou igual a zero, pois é invalido!");

    } else if ((a + b) > c) && ((b + c) > a) && ((a + c) > b) {
        println!("Os lados formam um triângulo!\n");

        if (a == b) && (a == c) {
            println!("O triangulo é equilátero.\n");

        } else if (a == b && a != c) || (b == c && b != a) || (c == a && c != b) {
        // A lógcia pode ser resumida assim: "a == b || b == c || a == c", pois caso os 3 lados fossem iguais, eles cairiam no primeiro if, entao teoricamente nao tem a necessidade de verificar se o terceiro lado é diferente, apenas verificar se 2 dos 3 lados sao iguais.
            println!("O triangulo é isóceles.\n");

        } else {
            println!("O triangulo é escaleno.");
        }

    } else {
        println!("Não é possível formar um triangulo com esses lados.")
    }

}

fn main() {
    
    // lados de um trinagulo:
    let a:i32 = 4;
    let b:i32 = 4;
    let c:i32 = 4;

    classificar_triangulo(a,b,c);
}
