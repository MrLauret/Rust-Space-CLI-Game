use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use serde_json::Serializer;
use std::io::BufWriter;
use crate::services::game_state::GameState;
use std::{fs::{self, File}, io::{Read}};
use std::io;
use crate::utils::consola;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entrada {
    pub fecha: DateTime<Local>,
    pub descripcion: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MisionInfo {
    pub informe: Vec<Entrada>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventInfo {
    pub informe: Vec<Entrada>
}

pub trait Informe {
    fn informe_mut(&mut self) -> &mut Vec<Entrada>;

    fn entrada(&mut self, descripcion: String) {
        self.informe_mut().push(Entrada {
            fecha: Local::now(),
            descripcion: descripcion }
        );
    }
}
impl Informe for MisionInfo {
    fn informe_mut(&mut self) -> &mut Vec<Entrada> { &mut self.informe }
}
impl Informe for EventInfo {
    fn informe_mut(&mut self) -> &mut Vec<Entrada> { &mut self.informe }
}

impl MisionInfo {
    pub fn new() -> Self {
        MisionInfo { informe: Vec::new() }
    }
}
impl EventInfo {
    pub fn new() -> Self {
        EventInfo { informe: Vec::new() }
    }
}

impl GameState {
    pub fn guardar_datos(&self) {
        let mut input = String::new();

        consola::limpiar_consola();
        println!("===== GUARDAR DATOS =====");
        println!("0. Salir");
        println!("+ Nombre del archivo al que se va a guardar");

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