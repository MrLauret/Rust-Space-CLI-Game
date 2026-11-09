pub enum Rol {
    Medico = 0,
    Ingeniero = 1,
    Capitan = 2,
}

pub enum Estado {
    Activo = 0,
    Herido = 1,
    Incapacitado = 2
}

pub struct Tripulante {
    pub nombre: String,
    pub rol: Rol,
    pub salud: u8,
    pub estado: Estado
}

impl Tripulante {
    pub fn from(nombre: String, rol: Rol) -> Tripulante {
        Tripulante {
            nombre: nombre,
            rol: rol,
            salud: 100,
            estado: Estado::Activo,
        }
    }
}