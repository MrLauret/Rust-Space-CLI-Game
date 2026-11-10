use serde::Serialize;
use serde_json::Serializer;
use std::io::BufWriter;
use crate::services::motor_mision::Motor;
use std::{fs::{self, File}, io::{Read}};
use std::io;
use crate::utils::consola;

impl Motor {
    pub fn guardar_datos(&self) {
        let mut input = String::new();

        consola::limpiar_consola();
        println!("===== GUARDAR DATOS =====");
        println!("0. Salir");
        println!("+ Nombre del archivo al que se va a guardar");

        io::stdin()
            .read_line(&mut input)
            .expect("Fallo al leer");

        if input == "0".to_string() {
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

    pub fn cargar_datos(path: String) -> Result<Motor, String>{
        let mut file = match File::open(path) {
            Err(_) => return Err("Partida no encontrada".to_string()),
            Ok(f) => {
                f
            }
        };

        let mut json_string = String::new();
        let _ = file.read_to_string(&mut json_string);

        let motor_data: Motor = match serde_json::from_str(&json_string) {
            Err(_) => return Err("Partida no encontrada".to_string()),
            Ok(m) => m
        };

        Ok(motor_data)
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