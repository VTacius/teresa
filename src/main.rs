use futures_util::{Stream, StreamExt};

use std::sync::{Mutex, Arc};

use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter, sse::Event};

type Canal = Arc<Mutex<Vec<mpsc::UnboundedSender<u64>>>>;

fn difundir_mensaje(ts: u64, canal: &Canal) {
    //let mensaje = format!("> {}", ts);
    canal.lock().unwrap().retain(|tx| {
        tx.send(ts).is_ok()
    });
} 

fn recibir_mensaje(canal: Canal) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    
    canal.lock().unwrap().push(tx);

    rx.map(|mensaje| {Ok(Event::default().data(format!("> {}", mensaje)))})
}

fn inicio () -> Result<warp::hyper::Response<&'static str>, warp::http::Error>{
    warp::http::Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(INDEX_HTML)
}

fn backend(ts :u64) -> u64 {
    println!("Estoy accediendo a backend");
    (ts + 1) % 25
}

#[tokio::main]
async fn main() {

    let canal = Arc::new(Mutex::new(Vec::new()));
    let canal = warp::any().map(move || canal.clone());

    let cartero = warp::path("cartero")
        .and(warp::path::param::<u64>())
        .and(canal.clone())
        .map(|ts, canal| {
            let ts = backend(ts);
            difundir_mensaje(ts, &canal);
            warp::reply()
        });
    
    let buzon = warp::path("buzon")
        .and(canal)    
        .map(|canal|{
            let event_stream = recibir_mensaje(canal);
            let event_stream = warp::sse::keep_alive().stream(event_stream);
            warp::sse::reply(event_stream)
        });
    
    let index = warp::path::end().map(inicio);

    let routes = index.or(buzon).or(cartero);
   
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080)).await;

}

static INDEX_HTML :&str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title> Warp, ese concepto </title>
    </head>
    <body>
        <h2> Prueba de contenido </h2>
        <div id="estado">Conectando...</div>
        <div id="contenido"></div>
        <script type="text/javascript">
            
            let uri = 'http://' + location.host + '/buzon';
            let sse = new EventSource(uri);

            function mostrarMensaje(data) {
                console.log(data);
                let linea = document.createElement('p');
                linea.innerText = data;
                contenido.appendChild(linea);
            }

            sse.onopen = function(){
                estado.innerHTML = "Conectado al servidor";
            }

            sse.onmessage = function(mensaje) {
                mostrarMensaje(mensaje.data);
            }

        </script>
    </body>
</html>

"#;