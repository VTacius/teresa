use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Dispositivo {
    //tipo' => $tipo,
    //nombre' => $nombre,
    pub latitude: f64, 
    pub longitude: f64, 
    pub hostname: String,
    pub estado: bool 
    //duracion' => $d['last_polled']
}

impl Dispositivo {
    pub fn new(hostname :String, latitude :f64, longitude :f64) -> Dispositivo {
        let estado = true;
        Dispositivo { latitude, longitude, hostname, estado }
    }
}

pub type ListaDispositivos = Vec<Dispositivo>;

pub fn obtener_dispositivos() -> ListaDispositivos {
    vec![
        Dispositivo::new("colmena".into(), 13.724791383896932, -88.93766858263731),
        Dispositivo::new("lirio".into(), 13.734020931658755, -89.47048792178529),
        Dispositivo::new("landaverde".into(), 13.768230880995919, -89.03663635076009),
    ]
}