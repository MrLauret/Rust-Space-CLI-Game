use serde::Serialize;
use serde_json::Serializer;
use std::io::BufWriter;
use crate::services::game_state::GameState;
use std::{fs::{self, File}, io::{Read}};
use std::io;
use crate::utils::consola;

impl GameState {
    pub fn guardar_datos(&self) {
        consola::limpiar_consola();
        let mut input = String::new();
        let partidas = match GameState::leer_partidas() {
            Err(e) => {println!("{e}"); return;},
            Ok(v) => v
        };

        let mut count = 0;
        println!("===== CARGAR PARTIDA =====");
        for partida in partidas.into_iter() {
            count += 1;
            println!("{count}. {}", partida.trim_end_matches(".json"))
        }
        println!("0. Salir");
        println!("+ Nombre del archivo al que se va a guardar (Si no coincide con ninguna se guarda en una partida nueva)");

        io::stdin()
            .read_line(&mut input)
            .expect("Fallo al leer");

        let input = input.trim();

        if input == "0".to_string() {
            return;
        } if input == "".to_string() {
            println!("- Debes poner un nombre a la partida");
            consola::enter_para_continuar();
            return;
        }

        let ruta = format!("saves/{input}.json");
        
        let file = File::create(&ruta)
            .map_err(|e| format!("No se pudo crear el archivo {}: {}", ruta, e))
            .expect("No se pudo crear el archivo");
        
        let writer = BufWriter::new(file);
        let mut serializer = Serializer::new(writer);
        
        self.serialize(&mut serializer)
            .map_err(|e| format!("Error de serialización: {e}"))
            .expect("Error de serialización");
        
        println!("Partida guardada");
        consola::enter_para_continuar();
    }

    pub fn cargar_datos(path: String) -> Result<GameState, String>{
        let mut file = match File::open(path) {
            Err(_) => return Err("Partida no encontrada".to_string()),
            Ok(f) => {
                f
            }
        };

        let mut json_string = String::new();
        let _ = file.read_to_string(&mut json_string);

        let game_data: GameState = match serde_json::from_str(&json_string) {
            Err(_) => return Err("Partida no encontrada".to_string()),
            Ok(m) => m
        };

        Ok(game_data)
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
        self.planetas = partida_cargada.planetas;

        println!("Partida cargada");
        consola::enter_para_continuar();
    }

    pub fn leer_partidas() -> Result<Vec<String>, String> {
        let mut nombres = Vec::new();

        let entradas = match fs::read_dir("saves") {
            Err(_) => return Err("No se encontró directorio de saves".to_string()),
            Ok(e) => {e}
        };

        let lista_archivos: Vec<fs::DirEntry> = entradas
            .filter_map(|res| res.ok())
            .collect();

        for entrada in lista_archivos {
            nombres.push(entrada.file_name().to_string_lossy().into_owned());
        }

        Ok(nombres)
    }
}