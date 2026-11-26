use core::fmt;
use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Rol {
    Medico,
    Ingeniero,
    Capitan,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Estado {
    Activo,
    Incapacitado,
    Ocupado
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tripulante {
    pub nombre: String,
    pub id: u32,
    pub rol: Rol,
    pub salud: u8,
    pub estado: Estado
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tripulacion {
    pub tripulacion: Vec<Tripulante>,
}

impl Display for Rol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let texto = match self {
            Rol::Medico => "Médico",
            Rol::Ingeniero => "Ingeniero",
            Rol::Capitan => "Capitán",
        };
        write!(f, "{}", texto)
    }
}

impl Display for Estado {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let texto = match self {
            Estado::Activo => "Activo",
            Estado::Incapacitado => "Incapacitado",
            Estado::Ocupado => "Ocupado"
        };
        write!(f, "{}", texto)
    }
}


impl Tripulante {
    pub fn from(nombre: String, id:u32, rol: Rol) -> Tripulante {
        Tripulante {
            nombre: nombre,
            id: id,
            rol: rol,
            salud: 100,
            estado: Estado::Activo,
        }
    }

    pub fn curar(&mut self) {
        self.salud = 100;
        self.estado = Estado::Activo
    }
}

impl Tripulacion {
    pub fn new() -> Self {
        Tripulacion { tripulacion: Vec::new() }
    }

    pub fn buscar_tripulante(&self, nombre:String) -> Option<&Tripulante>  {
        self.tripulacion.iter().find(|t| t.nombre == nombre)
    }

    pub fn check_medico(&self) -> bool {
        let medico = self.buscar_tripulante("Médico Medicalez".to_string()).unwrap();
        if medico.estado == Estado::Activo {
            return true;
        }
        false
    }
}