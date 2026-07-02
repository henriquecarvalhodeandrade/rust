// Constantes são variáveis que nao devem mudar nunca, veja:
// Normalmente ficam no escopo global, elas nao podem ser mutaveis
const TIPO_DE_DADO:i8 = 2;

// uma variavel estatica pode ser mutavel
static mut UMA_VARIAVEL_ESTATICA:i8 = 3;



fn main() {
    println!("Valor da Constante = {}", TIPO_DE_DADO);
    println!("Valor da Static = {}", UMA_VARIAVEL_ESTATICA);
    imprime();
}

fn imprime() {
    UMA_VARIAVEL_ESTATICA = 9;
    UMA_VARIAVEL_ESTATICA = 5
    println!("Valor da Constante = {}", TIPO_DE_DADO);
    println!("Valor da Static = {}", UMA_VARIAVEL_ESTATICA);
}

