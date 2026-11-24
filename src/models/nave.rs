use crate::models::tripulante::{Rol, Tripulante};
use crate::utils::consola::{enter_para_continuar, limpiar_consola};
use serde::{Serialize, Deserialize};
use std::{io};

#[derive(Serialize, Deserialize, Debug)]
pub enum Estado {
    Bueno,
    Medio,
    Malo
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Escudo {
    Operativo,
    Desactivado
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nave {
    pub combustible: u8,
    pub oxigeno: u8,
    pub energia: u8,
    pub integridad: u8,
    pub escudos: Escudo,
    pub distancia_al_destino: u32,
    pub tripulantes: Vec<Tripulante>,
    pub estado_general: Estado
}

impl Nave {
    pub fn new() -> Nave {
        Nave {
            combustible: 100,
            oxigeno: 100,
            energia: 100,
            integridad: 100,
            escudos: Escudo::Operativo,
            distancia_al_destino: 0,
            tripulantes: Nave::comienzo_tripulacion(),
            estado_general: Estado::Bueno
        }
    }

    pub fn comienzo_tripulacion() -> Vec<Tripulante> {
        vec![
            Tripulante::from(String::from("Medico Medicalez"), 1, Rol::Medico),
            Tripulante::from(String::from("John I. Gieneria"), 2, Rol::Ingeniero),
            Tripulante::from(String::from("Capi Tan Rodriguez"), 3, Rol::Capitan),
        ]
    }

    pub fn ver_estado(&self) {
        limpiar_consola();
        println!("===ESTADO DE LA NAVE===");
        println!("+Escudos:        {}", self.escudos);
        println!("+Combustible:    {}", self.combustible);
        println!("+Oxigeno:        {}", self.oxigeno);
        println!("+Integridad:     {}", self.integridad);
        println!("+Estado general: {}\n", self.estado_general);
        enter_para_continuar()
    }

    pub fn ver_tripulacion(&mut self) {
        limpiar_consola();
        println!("===TRIPULACION===");
        let mut input = String::new();
        for tripulante in self.tripulantes.iter() {
            println!("{}. {}", tripulante.id, tripulante.nombre);
        }
        println!("0. Salir");
        println!("Escribe su id para ver al tripulante");

        io::stdin()
            .read_line(&mut input)
            .expect("Fallo al leer");

        let input_fixed: u32 = match input.trim().parse() {
            Err(_) => {
                println!("- Solo se aceptan números");
                enter_para_continuar();
                return;
            }
            Ok(v) => v
        };

        if (0..3).contains(&input_fixed) {
            Continuar Luego
        }
    }
}

impl std::fmt::Display for Estado {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Estado::Bueno => write!(f, "Bueno"),
            Estado::Medio => write!(f, "Medio"),
            Estado::Malo => write!(f, "Malo"),
        }
    }
}

impl std::fmt::Display for Escudo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Escudo::Operativo => write!(f, "Operativos"),
            Escudo::Desactivado => write!(f, "Desactivados"),
        }
    }
}