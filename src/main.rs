mod backend;

use futures_util::{Stream, StreamExt};

use std::sync::{Mutex, Arc};

use uuid::Uuid;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter, sse::Event, http::Response};

use backend::{ListaDispositivos, obtener_dispositivos, guardar_poller};
type Canal = Arc<Mutex<Vec<mpsc::UnboundedSender<ListaDispositivos>>>>;

fn difundir_mensaje(devices: ListaDispositivos, canal: &Canal) {
    //let mensaje = format!("> {}", ts);
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

#[tokio::main]
async fn main() {

    let canal = Arc::new(Mutex::new(Vec::new()));
    let canal = warp::any().map(move || canal.clone());

    let cartero = warp::path("cartero")
        .and(warp::path::param::<Uuid>())
        .and(warp::path::param::<f64>())
        .and(canal.clone())
        .map(|uuid, ts, canal| {
            // Antes de todo, enviamos también su ts para que quede guardado
            guardar_poller(ts, uuid); 
            // Obtenemos los dispositivos
            let dispositivos = obtener_dispositivos(ts, uuid);
            // Enviamos el mensaje
            difundir_mensaje(dispositivos, &canal);
            // Devolvemos un resultado al poller
            Response::builder()
                .status(201)
                .header("X-uuid-poller", uuid.to_string())
                .body("")
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
