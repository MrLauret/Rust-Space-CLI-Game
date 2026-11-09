use crate::models::tripulante::Tripulante;

pub enum Estado {
    Bueno = 0,
    Medio = 1,
    Malo = 2
}

pub enum Escudos {
    Operativos = 0,
    Desactivados = 1,
}

pub struct Nave {
    pub combustible: u32,
    pub oxigeno: u32,
    pub energia: u32,
    pub integridad: u32,
    pub escudos: Escudos,
    pub distancia_al_destino: u32,
    pub tripulantes: Vec<Tripulante>,
    pub estado_general: Estado
}