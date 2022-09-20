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

fn inicio () -> Result<warp::hyper::Response<&'static str>, warp::http::Error>{
    warp::http::Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(INDEX_HTML)
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
    
    let index = warp::path::end().map(inicio);

    let routes = index.or(buzon).or(cartero);
   
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080)).await;

}

static INDEX_HTML :&str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title> Mapa de Estado para dispositivos </title>
        <link rel="stylesheet" href="https://unpkg.com/modern-css-reset/dist/reset.min.css" />
        <link rel="stylesheet" href="https://unpkg.com/leaflet@1.8.0/dist/leaflet.css" integrity="sha512-hoalWLoI8r4UszCkZ5kL8vayOGVae1oxXe/2A4AO6J9+580uKHDO3JdHb7NzwwzK5xr/Fs0W40kiNHxM9vyTtQ==" crossorigin="" />
        <style type="text/css">
            body {
                height: 600px;
            }
            .mapa {
                height: 590px;
            }
            
        </style>
    </head>
    <body>
        <div id="mapa"></div>
    </body>
    <script src="https://unpkg.com/leaflet@1.8.0/dist/leaflet.js" integrity="sha512-BB3hKbKWOc9Ez/TAwyWxNXeoV9c1v6FIeYiBieIWkpLjauysF18NzgR1MBNBXf8/KABdlkX68nAhlwcDFLGPCQ==" crossorigin=""></script>
    <script type="text/javascript">
        // Los valores por defecto para el mapa
        var default_lat = "13.8054";
        var default_lng = "-88.9069";
        var default_zoom = 10;

        /**  Esto es básicamente la configuración inicial del mapa */
        var mapa = L.map('mapa').setView([default_lat, default_lng], default_zoom);

        /** Y esto hará que aparezca por arte de magia */
        L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(mapa);

        
        /** Aca empezamos con los datos */
        let uri = 'http://' + location.host + '/buzon';
        let sse = new EventSource(uri);
            
        function mostrarEstablecimientos(data) {
            console.log(data);
            let contenido = JSON.parse(data);
            console.log(contenido);
            contenido.forEach(point => {
                L.marker([point.latitude, point.longitude]).addTo(mapa)
                    .bindPopup('A pretty CSS3 popup.<br> Easily customizable.');
                });
            

        }

        sse.onmessage = function(mensaje) {
            mostrarEstablecimientos(mensaje.data);
        }

    </script>
</html>
"#;