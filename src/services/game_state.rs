use crate::models::nave::Nave;
use crate::models::planeta::Planeta;
use crate::utils::consola;
use crate::services::datos::{MisionInfo, EventInfo};
use std::{io};
use serde::{Serialize, Deserialize};

const MENU_TEXT: &str =
"===== MISSION CONTROL =====
1. Ver estado de la nave
2. Ver tripulación
3. Consultar destino
4. Continuar misión
5. Consultar informe de la misión
6. Consultar informe de eventos
7. Guardar partida
8. Cargar partida
0. Salir
";

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub nave: Nave,
    pub planetas: Vec<Planeta>,
    pub mision_info: MisionInfo,
    pub event_info: EventInfo
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            nave: Nave::new(),
            planetas: Planeta::start(),
            mision_info: MisionInfo::new(),
            event_info: EventInfo::new()
        }
    }

    pub fn cargar_into(&mut self) {
        let partida: u32;
        let partidas = match GameState::leer_partidas() {
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
        let partida_cargada: GameState = match GameState::cargar_datos(path) {
            Err(e) => {println!("{e}"); return;},
            Ok(p) => p
        };

        self.nave = partida_cargada.nave;
        self.planetas = partida_cargada.planetas
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