// Exercício 2 (Fácil)
// Desenvolva um programa que declare uma variável contendo a nota de um aluno, variando
// de 0 a 10. Utilize uma estrutura if-else para informar a situação do aluno de acordo
// com as seguintes regras: nota maior ou igual a 6, o aluno está aprovado; nota entre
// 4 e 5,9, o aluno está em recuperação; nota menor que 4, o aluno está reprovado.
// Exiba a situação correspondente.

fn avaliacao_nota(nota: f32) {
    if nota < 0.0 || nota > 10.0 {
        println!("A nota informada é inválida.");
    } else if nota >= 6.0 {
        println!("Aluno aprovado! Sua nota foi {}", nota);
    } else if nota >= 4.0 {
        println!("Aluno está em recuperação. Sua nota foi {}", nota);
    } else {
        println!("Aluno reprovado. Sua nota foi {}", nota);
    }
}

fn main() {
    let nota:f32 = 3.0;
    
    avaliacao_nota(nota);
}
