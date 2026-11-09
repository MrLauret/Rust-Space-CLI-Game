pub enum Temperatura {
    Segura = 0,
    Peligrosa = 1
}

pub enum Atmosfera {
    Respirable = 0,
    NoRespirable = 1
}

pub enum Radiacion {
    Baja = 0,
    Media = 1,
    Alta = 2
}

pub enum Recurso {
    Combustible = 0,
    Alimento = 1,
    Minerales = 2
}

pub struct Planeta {
    pub atmosfera: Atmosfera,
    pub temperatura: Temperatura,
    pub radiación: Radiacion,
    pub distancia: u32,
    pub recursos: Vec<Recurso>,
    pub riesgo: u32
}