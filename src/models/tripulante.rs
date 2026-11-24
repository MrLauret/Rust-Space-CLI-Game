use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Rol {
    Medico = 0,
    Ingeniero = 1,
    Capitan = 2,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Estado {
    Activo = 0,
    Herido = 1,
    Incapacitado = 2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tripulante {
    pub nombre: String,
    pub id: u32,
    pub rol: Rol,
    pub salud: u8,
    pub estado: Estado
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
}