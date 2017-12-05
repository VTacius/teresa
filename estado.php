<?php

/* Obtenemos los datos desde nuestra fuente primaria: Por ahora, la API de libreNMS */
$curl = curl_init();
require_once('config.php');
curl_setopt_array($curl, array(
    CURLOPT_URL => 'http://' . $sapi . '/api/v0/devices',
    CURLOPT_HTTPHEADER => ['X-Auth-Token: ' . $token],
    CURLOPT_RETURNTRANSFER => true
));

$respuesta = curl_exec($curl);
curl_close($curl);

$datos = json_decode($respuesta, true);

/* Acá guardaremos los datos, la salida final que hemos de enviar al cliente web */
$salida = array();

/* De la descripción en la base de datos, hacemos {tipo} - {descripción} */
function parsea_descripcion ($descripcion){
    if(preg_match('/^(\w+)\s\-\s(.+)/', $descripcion, $res) === 1){
        return [$res[1], $res[2]];
    } else {
        return false;
    }

}

/* De la localidad en la base de datos, hacemos [{latitude},{longitude}] */
function parsea_localidad ($localidad){
    if(preg_match('/\[(.+),(.+)\]/', $localidad, $res) === 1){
        return [$res[1], $res[2]];
    } else {
        return false;
    }
}

/* Procesa los dispositivos y crea un diccionario con los datos que en verdad necesitemos */
foreach($datos['devices'] as $d){
    list($latitude, $longitude) = parsea_localidad($d['location']);
    list($tipo, $nombre) = parsea_descripcion($d['purpose']);
    if (is_string($latitude) and is_string($tipo)){
    	$salida[$d['hostname']] = [ 
    	    'tipo' => $tipo,
    	    'nombre' => $nombre,
    	    'latitude' => $latitude,
    	    'longitude' => $longitude,
            'hostname' => $d['hostname'],
    	    'estado' => $d['status'],
    	    'duracion' => $d['last_polled']
    	];
        
    }
}

print json_encode($salida); 
?>
