use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entrada {
    pub fecha: DateTime<Local>,
    pub descripcion: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MisionInfo {
    pub informe: Vec<Entrada>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventInfo {
    pub informe: Vec<Entrada>
}

pub trait Informe {
    fn informe_mut(&mut self) -> &mut Vec<Entrada>;

    fn entrada(&mut self, descripcion: String) {
        self.informe_mut().push(Entrada {
            fecha: Local::now(),
            descripcion: descripcion }
        );
    }
}
impl Informe for MisionInfo {
    fn informe_mut(&mut self) -> &mut Vec<Entrada> { &mut self.informe }
}
impl Informe for EventInfo {
    fn informe_mut(&mut self) -> &mut Vec<Entrada> { &mut self.informe }
}

impl MisionInfo {
    pub fn new() -> Self {
        MisionInfo { informe: Vec::new() }
    }
}
impl EventInfo {
    pub fn new() -> Self {
        EventInfo { informe: Vec::new() }
    }
}