use std::fs;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::BackendError;

#[derive(Clone, Deserialize, Serialize)]
pub struct Servidor {
    pub tipo: String,
    pub estado: bool,
    pub ttl: u32,
    pub rtt: u32
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Dispositivo {
    pub tipo: String,
    pub estado: String,
    pub nombre: String,
    pub servidores: Vec<Servidor>,
    pub coordenadas: (f64, f64)
}

pub type ListaDispositivos = Vec<Dispositivo>;

pub fn obtener_dispositivos(_ts: f64, _uuid: Uuid) -> Result<ListaDispositivos, BackendError> {
    let data = fs::read_to_string("./src/datos_minsal.json")?;
    let contenido: ListaDispositivos = serde_json::from_str(&data).expect("Nada, error al deserializar");

    Ok(contenido)
}

pub fn guardar_poller(ts: f64, uuid: Uuid) {
    println!("{} - {}", ts, uuid);
}