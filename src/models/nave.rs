use crate::models::tripulante::{Rol, Tripulante};
use crate::utils::consola;

pub enum Estado {
    Bueno = 0,
    Medio = 1,
    Malo = 2
}

pub enum Escudo {
    Operativo = 0,
    Desactivado = 1,
}

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
            Tripulante::from(String::from("Medico Medicalez"), Rol::Medico),
            Tripulante::from(String::from("John I. Gieneria"), Rol::Ingeniero),
            Tripulante::from(String::from("Capi Tan Rodriguez"), Rol::Capitan),
        ]
    }

    pub fn ver_estado(&self) {
        consola::limpiar_consola();
        println!("===ESTADO DE LA NAVE===");
        println!("+Escudos:        {}", self.escudos);
        println!("+Combustible:    {}", self.combustible);
        println!("+Oxigeno:        {}", self.oxigeno);
        println!("+Integridad:     {}", self.integridad);
        println!("+Estado general: {}\n", self.estado_general);
        consola::enter_para_continuar()
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