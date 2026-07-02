// 3. Cálculo de média escolar: Faça um programa que declare três variáveis do tipo decimal para armazenar notas de um aluno. O programa deve calcular a soma dessas notas e, em seguida, calcular a média aritmética. Ao final, exiba todas as notas, a soma e a média calculada. Utilize comentários para explicar a lógica.

fn calcular_media_escolar(x:f32, y:f32, z:f32){
    let soma:f32, media:f32;

    soma = x + y + z
    media = soma / 3

    println!("A soma das notas é: {}, já a médias destas notas é: {}", soma, media)
    
}

fn main() {

    let x:f32 = 10
    let y:f32 = 6
    let z:f32 = 4

    calcular_media_escolar(x,y,z)

    
}
