import './css/fuentes.css';
import './css/reset.css';
import 'leaflet/dist/leaflet.css';
import './style.css';

import L from 'leaflet';

import  { contenidoEstablecimiento} from './ui/popup';
import {configurarIcono} from './ui/iconos';

// Ahora si empezamos a hacer el mapa
let mapa = L.map("mapa").setView([13.8054, -88.9069], 10);

// Le ponemos algo bonito de ver
const tiles = L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
}).addTo(mapa);


function mostrarEstablecimientos(establecimientos){
    establecimientos.map((establecimiento) => {
        let icon = configurarIcono(establecimiento.tipo, establecimiento.estado);
        let contenido = contenidoEstablecimiento(establecimiento.nombre, establecimiento.servidores);
        L.marker(establecimiento.coordenadas).setIcon(icon).addTo(mapa).bindPopup(contenido);
    });

}

// Acá empezamos con los datos
let uri = 'http://' + location.host + '/buzon';
let sse = new EventSource(uri);
    
sse.onmessage = function(mensaje) {
    let contenido = JSON.parse(mensaje.data);
    console.log("Mensaje recibido");
    mostrarEstablecimientos(contenido);
}
    