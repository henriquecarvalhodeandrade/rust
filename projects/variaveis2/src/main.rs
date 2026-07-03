fn main() {
    // um i8 = int8, significa um integer (inteiro de até 8 bits), ou seja, os números inteiros que possuem até 8 bits vai do -128 ao 127, tente mudar x para 128 e veja que dará erro.
    let x:i8 = 127;

    // Assim como i8, para os demais int, entao: i16 = int16 (inteiro de até 16 bits), vai do -32768 ao 32767
    let y:i16 = 32767;

    // Assim por diante para os demais ints

    // Para tipos unsigned, nao podemos colocar valores negativos:
    // let z:u8 = -127; // Veja que esta dando erro!

    // tipos de dados: i(int), f(float), char, bool, str, u(unsigned), isize(tamanho de arquitetura), struct, enum, tuple, array, slice, String, Vec, Option, Result, HashMap, Matriz ( [] ), entre outros...

}
