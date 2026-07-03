// 4. Cálculo de área e perímetro: Crie um programa que armazene a largura e a altura de um retângulo em variáveis numéricas. Depois, calcule a área e o perímetro desse retângulo utilizando operadores matemáticos. Exiba todos os valores no terminal de forma organizada e com comentários explicativos no código.


fn calcula_area_e_perimetro_retangulo(base: f32, altura: f32){

    let area:f64 = (base * altura).into();
    let perimetro:f64 = ((base * 2.0) + (altura * 2.0)).into();

    println!("Após os calculos, a área do retngulo é: {} e o perímetro do retangulo é: {}", area, perimetro);

}

fn main() {
    let base:f32 = 10.0;
    let altura:f32 = 21.0;

    calcula_area_e_perimetro_retangulo(base, altura);

}
