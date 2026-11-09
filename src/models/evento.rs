pub enum Situacion {
    
}

pub enum Opcion {

}

pub enum Condicion {

}

pub enum Consecuencias {
    
}

pub struct Evento {
    pub situacion: Situacion,
    pub opciones: Vec<Opcion>,
    pub condiciones: Vec<Condicion>,
    pub consecuencias: Vec<Consecuencias>
}