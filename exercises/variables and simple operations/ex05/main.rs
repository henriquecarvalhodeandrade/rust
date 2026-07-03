// 5. Simulação de salário com bônus e descontos: Desenvolva um programa que armazene em variáveis o salário base de um funcionário, o valor de um bônus recebido e o valor de um desconto aplicado. O programa deve calcular o salário final após adicionar o bônus e subtrair o desconto. Além disso, calcule qual porcentagem do salário base foi descontada. Exiba todos os valores e resultados no terminal, utilizando comentários para documentar cada etapa do processo.

fn calcular_salario_final(salario:f32, bonus:f32, descontos:f32){
    let salario_final:f32;
    let salario_descontado:f32;

    salario_final = salario + bonus - descontos;
    salario_descontado = (descontos) / (salario / 100.0);

    println!("O salario liquido foi de: {}, foi descontado {} reais de seu salario total, ou seja, {}% de salário descontado.", salario_final, descontos, salario_descontado);

}

fn main() {
    let salario:f32 = 1500.00;
    let bonus:f32 = 500.00;
    let descontos:f32 = 600.00;

    calcular_salario_final(salario, bonus, descontos);
}
