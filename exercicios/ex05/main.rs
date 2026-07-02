// 5. Simulação de salário com bônus e descontos: Desenvolva um programa que armazene em variáveis o salário base de um funcionário, o valor de um bônus recebido e o valor de um desconto aplicado. O programa deve calcular o salário final após adicionar o bônus e subtrair o desconto. Além disso, calcule qual porcentagem do salário base foi descontada. Exiba todos os valores e resultados no terminal, utilizando comentários para documentar cada etapa do processo.

fn calcular_salario_final(salario:f32, bonus:f32, descontos:f32;){
    let salarioFinal:f32;
    let salarioDescontado:f32;



    salarioFinal = salario + bonus + descontos;
    salarioDescontado = (salario - desconto) / (salario / 100)

    println!("O salario liquido foi é de: {}, foi descontado {}% de seu salario no total.", salarioFinal, salarioDescontado);

}

fn main() {
    let salario:f32 = 1500 , bonus:f32 = 500, descontos:f32 = 250;

    calcular_salario_final(salario, bonus, descontos);
}
