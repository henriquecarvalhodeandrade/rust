// A biblioteca padrão (std::io) gerencia operações de entrada e saída.
use std::io;
// O Trait 'Rng' (Random Number Generator) define métodos utilitários para gerar números.
use rand::Rng; 
// 'Ordering' é uma Enum cujas variantes são: Less (Menor), Greater (Maior) e Equal (Igual).
use std::cmp::Ordering; 

fn main() {
    println!("Adivinhe o número!"); // O sufixo '!' indica que println é um Macro, não uma função comum.

    // rand::thread_rng() obtém um gerador de números aleatórios local para a thread atual.
    // O método gen_range(min, max) gera um valor no intervalo inclusivo/exclusivo.
    // Nota histórica: Em versões recentes do crate rand, usa-se o intervalo como range (1..=100).
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    // O bloco 'loop' inicia uma estrutura de repetição infinita.
    loop {
        println!("Digite seu palpite:");

        // Cria uma nova String dinâmica, expansível e codificada em UTF-8 na memória Heap.
        // 'new()' é uma função associada ao tipo String.
        let mut palpite = String::new();

        // io::stdin() inicializa o manipulador de entrada padrão do sistema.
        // O método .read_line() recebe uma referência mutável (&mut) para gravar o texto do terminal.
        // O uso do '&' evita a cópia de dados, compartilhando o ponteiro de forma segura.
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a linha"); // OBRIGATÓRIO: read_line retorna um Result. expect trata o caso 'Err'.

        // Tratamento de tipos de dados e tratamento de erro elegante com Match:
        // O método .trim() remove espaços em branco e o caractere de nova linha '\n' (gerado pelo Enter).
        // O método .parse() tenta converter a string para o tipo numérico anotado explicitamente (: u32).
        // Usamos o conceito de Sombreamento (Shadowing) para reaproveitar o nome da variável 'palpite'.
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,     // Se a conversão der certo, retorna o número.
            Err(_) => continue, // Se falhar (ex: digitou letras), o '_' ignora o erro e reinicia o loop.
        };

        println!("Você chutou: {}", palpite);

        // O método .cmp() compara dois valores do mesmo tipo e retorna uma variante da Enum 'Ordering'.
        // O Match realiza o casamento de padrões de forma exaustiva sobre essas variantes.
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito Baixo!"),
            Ordering::Greater => println!("Muito Alto!"),
            Ordering::Equal => {
                println!("Parabéns, você acertou!");
                break; // Interrompe o fluxo do 'loop' infinito, encerrando o programa com sucesso.
            }
        }
    }
}