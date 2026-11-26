use crate::models::tripulante::{Rol, Tripulante, Tripulacion};
use crate::models::tripulante::Estado as EstadoTripulante;
use crate::utils::consola::{enter_para_continuar, limpiar_consola};
use serde::{Serialize, Deserialize};
use std::{io, vec};

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
pub enum Tarea {
    Curar {nombre: String, dia:u32}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nave {
    pub combustible: u8,
    pub oxigeno: u8,
    pub energia: u8,
    pub integridad: u8,
    pub escudos: Escudo,
    pub distancia_al_destino: u32,
    pub tripulantes: Tripulacion,
    pub estado_general: Estado,
    pub dia: u32,
    pub lista_tareas: Vec<Tarea>
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
            estado_general: Estado::Bueno,
            dia: 1,
            lista_tareas: Vec::new()
        }
    }

    pub fn comienzo_tripulacion() -> Tripulacion {
        Tripulacion { tripulacion: vec![
            Tripulante::from(String::from("Médico Medicalez"), 1, Rol::Medico),
            Tripulante::from(String::from("John I. Gieneria"), 2, Rol::Ingeniero),
            Tripulante::from(String::from("Doctor Zomboss"), 3, Rol::Capitan),
        ]}
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

    pub fn añadir_tarea(&mut self, tarea: Tarea) {
        self.lista_tareas.push(tarea);
    }

    pub fn ver_tripulacion(&mut self) {
        limpiar_consola();
        println!("===TRIPULACION===");
        let mut input = String::new();
        for tripulante in self.tripulantes.tripulacion.iter() {
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

        limpiar_consola();
        let mut input = String::new();
        let hay_medico = self.tripulantes.check_medico();
        let mut tarea: Option<Tarea> = None;

        for tripulate in self.tripulantes.tripulacion.iter_mut() {
            if tripulate.id != input_fixed {continue;}
            
            println!("==={}===", tripulate.nombre.to_uppercase());
            println!("-{}", tripulate.rol);
            println!("-{}", tripulate.estado);
            println!("-{}\n", tripulate.salud);
            println!("Curar un tripulante lo incapacita por el tiempo que tome en ser curado");
            println!("Solo puedes curar a un tripulante a la vez");

            let tiempo_curación: u32;
            if hay_medico {
                "1. Curar (2 días, hay médico)";
                tiempo_curación = 2;
            } else {
                "1. Curar (5 días, no hay médico)";
                tiempo_curación = 5;
            };
            println!("\n0. Salir");

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

            match input_fixed {
                0 => {limpiar_consola(); return;}
                1 => {
                        let dia_objetivo = self.dia + tiempo_curación;
                        tarea = Some(Tarea::Curar {
                            nombre: tripulate.nombre.clone(),
                            dia: dia_objetivo
                        });
                        tripulate.estado = EstadoTripulante::Incapacitado;
                        println!("-El tripulante {} será curado el día {}", tripulate.nombre, dia_objetivo);
                        enter_para_continuar();
                        limpiar_consola();
                        break;
                }
                _ => {
                    println!("-Escribe una opción válida");
                    return;
                }
            }
        }
        if let Some(t) = tarea {
            self.añadir_tarea(t);
        }
        println!("-No se encontró al tripulante");
        enter_para_continuar();
        limpiar_consola();
        return
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