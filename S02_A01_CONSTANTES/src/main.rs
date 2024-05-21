/*
*/

//const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;

//const UMA_HORA_EM_SEGUNDOS = 1 * 60 * 60;

const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 600;

fn main() {
    println!("Inicio do programa");
    println!("Escopo externo: {UMA_HORA_EM_SEGUNDOS}");

    const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60; // ESCOPO INTERNO
    println!("Escopo interno: {UMA_HORA_EM_SEGUNDOS}");

    let mut x = 5;
    println!("O valor de x é: {x}");

    x = UMA_HORA_EM_SEGUNDOS;
    println!("O valor de x agora é: {x}");
}
