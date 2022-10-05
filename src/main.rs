mod backend;
mod errors;

use futures_util::{Stream, StreamExt};
use serde::{Deserialize, Serialize};

use std::sync::{Mutex, Arc};

use log::error as errorlog;

use uuid::Uuid;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter, sse::Event, http::Response, path};

use backend::{ListaDispositivos, obtener_dispositivos, guardar_poller, Dispositivo};
type Canal = Arc<Mutex<Vec<mpsc::UnboundedSender<ListaDispositivos>>>>;

fn difundir_mensaje(devices: ListaDispositivos, canal: &Canal) {
    canal.lock().unwrap().retain(|tx| {
        tx.send(devices.clone()).is_ok()
    });
} 

fn recibir_mensaje(canal: Canal) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
   
    canal.lock().unwrap().push(tx);

    rx.map(|dispositivos| {Ok(Event::default().json_data(dispositivos).unwrap())})
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Mensaje  {
    uuid: Uuid,
    ts: f64
}

fn parsear_json_body() -> impl Filter<Extract = (Mensaje, ), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(512).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_env().unwrap();
    
    let canal = Arc::new(Mutex::new(Vec::new()));
    let canal = warp::any().map(move || canal.clone());

    let cartero = warp::post()
        .and(path("cartero"))
        .and(parsear_json_body())
        .and(canal.clone())
        .map(|mensaje: Mensaje, canal: Arc<Mutex<Vec<UnboundedSender<Vec<Dispositivo>>>>>| {
            // Antes de todo, enviamos también su ts para que quede guardado
            guardar_poller(mensaje.ts, mensaje.uuid); 
            // Obtenemos los dispositivos

            match obtener_dispositivos(mensaje.ts, mensaje.uuid) {
                Ok(dispositivos) => {
                    // Enviamos el mensaje
                    difundir_mensaje(dispositivos, &canal);
                    // Devolvemos un resultado al poller
                    Response::builder()
                        .status(201)
                        .header("X-uuid-poller", mensaje.uuid.to_string())
                        .body("")
                },
                Err(e) => {
                    errorlog!("{:?}", e);
                    Response::builder()
                        .status(500)
                        .body("")
                }
            }
        });
    
    let buzon = warp::path("buzon")
        .and(canal)    
        .map(|canal|{
            let event_stream = recibir_mensaje(canal);
            let event_stream = warp::sse::keep_alive().stream(event_stream);
            warp::sse::reply(event_stream)
        });
    

    let routes = buzon.or(cartero);
   
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080)).await;

}
