use std::{range, vec};

use rand::{RngExt, rng, seq::IndexedRandom};

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

impl Planeta {
    pub fn new() -> Planeta {
        let mut rng = rand::rng();

        Planeta {
            temperatura: Temperatura::random(),
            atmosfera: Atmosfera::random(),
            radiación: Radiacion::random(),
            distancia: rng.random_range(500..10000),
            recursos: Recurso::random(),
            riesgo: rng.random_range(0..100)
        }
    }

    pub fn start() -> Vec<Planeta> {
        let mut vec = Vec::new();

        for _ in 0..50 {
            vec.push(Planeta::new());
        }

        vec
    }
}

impl Temperatura {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..2) {
            0 => Temperatura::Segura,
            _ => Temperatura::Peligrosa
        }
    }
}

impl Atmosfera {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..2) {
            0 => Atmosfera::Respirable,
            _ => Atmosfera::NoRespirable
        }
    }
}

impl Radiacion {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..3) {
            0 => Radiacion::Alta,
            1 => Radiacion::Media,
            _ => Radiacion::Baja
        }
    }
}

impl Recurso {
    pub fn random() -> Vec<Self> {
        let mut vec = Vec::new();
        let mut rng = rand::rng();
        let mut count = 0;

        loop {
            count += 1;

            if count == 3 {
                return vec;
            }

            match rng.random_range(0..3) {
                0 => vec.push(Recurso::Minerales),
                1 => vec.push(Recurso::Alimento),
                _ => vec.push(Recurso::Combustible),
            }

            match rng.random_range(0..2) {
                0 => {return vec},
                _ => {continue}
            }
        }
    }
}