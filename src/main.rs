mod backend;

use futures_util::{Stream, StreamExt};

use std::sync::{Mutex, Arc};

use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter, sse::Event};

use backend::{ListaDispositivos, obtener_dispositivos};
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
        .and(warp::path::param::<u64>())
        .and(canal.clone())
        .map(|_ts, canal| {
            // Antes de todo, enviamos también su ts para que quede guardado
            // Obtenemos los dispositivos
            let dispositivos = obtener_dispositivos();
            difundir_mensaje(dispositivos, &canal);
            warp::reply()
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
