pub enum Rol {
    Medico = 0,
    Ingeniero = 1
}

pub enum Estado {
    Activo = 0,
    Herido = 1,
    Incapacitado = 2
}

pub struct Tripulante {
    pub nombre: String,
    pub rol: Rol,
    pub salud: u32,
    pub estado: Estado
}