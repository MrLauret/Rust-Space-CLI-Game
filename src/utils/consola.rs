use std::io::{self, Write};

pub fn limpiar_consola() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn enter_para_continuar() {
    let mut input = String::new();
    println!("-Presiona enter para continuar-");
    io::stdin().read_line(&mut input).expect("Fallo al leer");
    limpiar_consola();
}