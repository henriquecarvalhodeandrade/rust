/*

// Constantes são variáveis que nao devem mudar nunca, veja:
// Normalmente ficam no escopo global, elas nao podem ser mutaveis
const TIPO_DE_DADO:i8 = 2;

// uma variavel estatica pode ser mutavel
static mut UMA_VARIAVEL_ESTATICA:i8 = 3;

*/


/*       comentario multi-linha

o
iui
wdw

*/

fn main() {
    // Variáveis por padrao sao imutaveis, veja:

    // let x = 5;
    // let x += 6;
    // Vai dar erro!

    // Para torná-las mutaveis, fazemos assim:
    let mut x: i32 = 5;
    x += 6;

    println!("Hello, world! {}", x);

    /*

    // CONSTANTES E ESTATICOS   
    // Ao fazer mudanças em algumas variaveis, pode ocorrer erros de data racing, por usar variaveis estaticas em escopos de funções diferentes, entao para poder mudar elas mesmo assim, devemos informar que iremos usar ela de forma unsafe insegura)

    unsafe {
        UMA_VARIAVEL_ESTATICA = 4;
    }

    println!("Valor da Constante = {}", TIPO_DE_DADO);
    println!("Valor da Static = {}", UMA_VARIAVEL_ESTATICA);
    imprime();
    */

    // Shadowing é quando podemos declarar novas variáveis com o mesmo nome de uma outra já declarada.
    // Isso é útil quando queremos mudar o tipo de uma variável, por exemplo:

    let x = 5;
    println!("Valor de x = {} e seu endereço de memória = {:p}", x, &x);
    
    let x = x + 1;
    println!("Valor de x = {} e seu endereço de memória = {:p}", x, &x);
    
    let x = x * 2;
    println!("Valor de x = {} e seu endereço de memória = {:p}", x, &x);

    // Perceba que para cada variavel, foi sobreescrita em um espaço diferente de memória, ou seja, nao era o mesmo espaço chamado de x que foi mudado de valor, na verdade foram criados novos espaços chamados de x com esses valores diferentes.

}


/*
fn imprime() {
    // Devemos informar que queremos usar de forma unsafe!

    UMA_VARIAVEL_ESTATICA = 9;
    UMA_VARIAVEL_ESTATICA = 5;
    println!("Valor da Constante = {}", TIPO_DE_DADO);
    println!("Valor da Static = {}", UMA_VARIAVEL_ESTATICA);
}
*/

