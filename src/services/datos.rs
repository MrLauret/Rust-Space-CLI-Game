use crate::services::motor_mision::Motor;
use std::{fs::{self, File}, io::Read, string};

impl Motor {
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