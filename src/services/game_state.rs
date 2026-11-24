use crate::models::nave::Nave;
use crate::models::planeta::Planeta;
use crate::utils::consola;
use crate::models::informes::{MisionInfo, EventInfo};
use std::{io};
use serde::{Serialize, Deserialize};

const MENU_TEXT: &str =
"===== MISSION CONTROL =====
1. Ver estado de la nave
2. Ver tripulación
3. Consultar destino
4. Consultar planetas
5. Informe de la misión
6. Informe de eventos
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
                0 => {println!("===Mision Cancelada==="); break}
                1 => {self.nave.ver_estado()}
                2 => {self.nave.ver_tripulacion();}
                7 => {self.guardar_datos();}
                8 => {self.cargar_into();}
                _ => {
                    println!("-Escribe un número válido-");
                    consola::enter_para_continuar();
                    continue
                }
            }
        }
    }


}