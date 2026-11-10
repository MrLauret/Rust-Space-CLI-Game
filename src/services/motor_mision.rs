use crate::models::nave::Nave;
use crate::models::planeta::Planeta;
use crate::utils::consola;
use std::io;

const MENU_TEXT: &str =
"===== MISSION CONTROL =====
1. Ver estado de la nave
2. Ver tripulación
3. Consultar destino
4. Continuar misión
5. Consultar historial
6. Guardar partida
7. Cargar partida
0. Salir
";

pub struct Motor {
    pub nave: Nave,
    pub planetas: Vec<Planeta>
}

impl Motor {
    pub fn new() -> Motor {
        Motor {
            nave: Nave::new(),
            planetas: Planeta::start()
        }
    }

    pub fn start(&self) {
        consola::limpiar_consola();
        loop {
            println!("{MENU_TEXT}");
            let mut input = String::new();
            
            io::stdin()
                .read_line(&mut input)
                .expect("Fallo al leer");

            let input_sano: u32 = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => {
                    println!("-Escribe un número válido-");
                    consola::enter_para_continuar();
                    continue;
                }
            };

            match input_sano {
                0 => {
                    println!("===Mision Cancelada===");
                    break
                },
                1 => {self.nave.ver_estado()}
                _ => {
                    println!("-Escribe un número válido-");
                    consola::enter_para_continuar();
                    continue
                }
            }
        }
    }


}