use crate::models::nave::Nave;
use crate::models::planeta::Planeta;
use crate::utils::consola;
use std::{io};
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
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

    pub fn cargar_into(&mut self) {
        let partida: u32;
        let partidas = match Motor::leer_partidas() {
            Err(e) => {println!("{e}"); return;},
            Ok(v) => v
        };

        loop {
            let mut count = 0;
            consola::limpiar_consola();
            println!("===== CARGAR PARTIDA =====");
            for partida in partidas.clone().into_iter() {
                count += 1;
                println!("{count}. {}", partida.trim_end_matches(".json"))
            }
            println!("0. Salir");

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

            if input_sano == 0 {
                return;
            }

            partida = input_sano;
            break;
        }
        let partida_seleccionada = &partidas[(partida - 1) as usize];
        let path = format!("saves/{partida_seleccionada}");
        let motor_cargado: Motor = match Motor::cargar_datos(path) {
            Err(e) => {println!("{e}"); return;},
            Ok(p) => p
        };

        self.nave = motor_cargado.nave;
        self.planetas = motor_cargado.planetas
    }

    pub fn start(&mut self) {
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
                1 => {self.nave.ver_estado()},
                6 => {self.guardar_datos(); consola::limpiar_consola(); println!("Partida guardada");}
                7 => {self.cargar_into(); consola::limpiar_consola(); println!("Partida cargada");}
                _ => {
                    println!("-Escribe un número válido-");
                    consola::enter_para_continuar();
                    continue
                }
            }
        }
    }


}