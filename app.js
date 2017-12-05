/* 
 * Accedo a los datos 
 */
var api = '/estadio.php';
var tiempoRecarga = 20000;

var obtenerDatos = function(){
    return new Promise(function(resolve, reject){
    	console.log('Al menos hago promesa');
        var peticion = new XMLHttpRequest();
        peticion.open('GET', api, true);
    	peticion.onload = function(){
    		console.log(peticion.status);
    		if (peticion.status === 200){
    			var datos = JSON.parse(peticion.response);
    			console.log('Estoy en medio de la petición de datos');
                console.log(datos['10.20.40.1']);
    			resolve(datos);
    			peticion = null;
    		} else{
    			reject(new Error(peticion.statusText));
    		}
    	};
    
    	peticion.onerror = function(){
    		reject(new Error("Error al intentar acceder a los datos"));
    	};
        
        peticion.send();
    });

};

/* 
 * Creo el mapa. El mapa debe ser lo primer en cargar para que el usuario no se sienta mal, so
 */
/* Configurando algunos valores por defecto */
var default_lat = "13.8054";
var default_lng = "-88.9069";
var default_zoom = 10;
var tile_url = "{s}.tile.openstreetmap.org";

/* Esto es básicamente la configuración inicial del mapa */
var mapa = L.map('mapaid').setView([default_lat, default_lng], default_zoom);
L.tileLayer('//' + tile_url + '/{z}/{x}/{y}.png', {
    attribution: '&copy; <a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
}).addTo(mapa);

/*
 * Creo los íconos personalizados, algo de mucho valor por acá
 */
var marcas = {};
var data = {};

/* Configuración inicial y común a todos los íconos */
var LeafIcon = L.Icon.extend({
    options: {
        iconSize: [16, 16]
    }
});

/* Los íconos. */
var iconos = {
    hospitales: {
        down: new LeafIcon({iconUrl: 'images/hospitaldown.png'}),
        unk: new LeafIcon({iconUrl: 'images/hospitalunk.png'}),
        up: new LeafIcon({iconUrl: 'images/hospitalup.png'})
    },
    sibasi :  {
        down: new LeafIcon({iconUrl: 'images/sibasidown.png'}),
        unk: new LeafIcon({iconUrl: 'images/sibasiunk.png'}),
        up: new LeafIcon({iconUrl: 'images/sibasiup.png'})
    },
    almacen :  {
        down: new LeafIcon({iconUrl: 'images/almacendown.png'}),
        unk: new LeafIcon({iconUrl: 'images/almacenunk.png'}),
        up: new LeafIcon({iconUrl: 'images/almacenup.png'})
    },
    region :  {
        down: new LeafIcon({iconUrl: 'images/regiondown.png'}),
        unk: new LeafIcon({iconUrl: 'images/regionunk.png'}),
        up: new LeafIcon({iconUrl: 'images/regionup.png'})
    },
    otros :  {
        down: new LeafIcon({iconUrl: 'images/otrosdown.png'}),
        unk: new LeafIcon({iconUrl: 'images/otrosunk.png'}),
        up: new LeafIcon({iconUrl: 'images/otrosup.png'})
    },
    ucsf :  {
        down: new LeafIcon({iconUrl: 'images/ucsfdown.png'}),
        unk: new LeafIcon({iconUrl: 'images/ucsfunk.png'}),
        up: new LeafIcon({iconUrl: 'images/ucsfup.png'})
    }
};

/* Auxiliar para hacer legible los datos que nos envia el servidor para el atributo estado */
var configuraEstado = function(estado){
	if (estado == 0){
		return 'down';
	}else if (estado == 1){
		return 'up';
    }else {
		return 'unk';
	}
};

/* Auxiliar para formar el cambiante ícono: Se basa en tipo de marcador y estado actual */
var configuraIcono = function(dataObjeto){
    /* TODO: Establecer un tipo por defecto */
    var tipo = iconos.hasOwnProperty(dataObjeto.tipo) ? dataObjeto.tipo : 'sibasi';
    var icono = iconos[tipo][configuraEstado(dataObjeto.estado)];
    return icono;
};

/* Auxiliar que facilita la creación de un titulo para el popup de los marcadores */ 
var creaTituloMarca = function(dataObjeto){
    return '<b>' + dataObjeto.nombre + '</b>:<br>' + 
        'Estado: <b>' + configuraEstado(dataObjeto.estado) + '</b>' +
        ' desde <b>' + dataObjeto.duracion + '</b>';
};


/*
 * Creo por primera vez los íconos, precisamente al cargar la paǵina
 * TODO: ¿Deberíamos esperar a que el mapa estuviera cargado, entre otras cosas que podría esperar ?
 */


/* Hacemos la creación inicial de los marcadores */
var iniciaMarcadores = function(datos){
    console.log('inicializando marcadores');
	Object.keys(datos).forEach(function(clave){
		var host = datos[clave];
        marcas[host['hostname']] = L.marker([host.latitude, host.longitude], {icon: configuraIcono(host)})
            .addTo(mapa);
	    marcas[host['hostname']].bindPopup(creaTituloMarca(datos[clave]));
	    /* TODO: Debe ser una función bien bonita que incluso pudiera poner valores por defecto, y modificarlos después */
    });
   	
    /* Es decir, la data global se llena con los datos obtenidos desde la primera llamada al servidor */
    data = datos; 
};

    
/* Actualizamos sólo aquellas marcas que de veras lo requieran */
var actualizaMarcadores = function(datos){
    console.log('actualizando marcadores');
    console.log(data['10.20.40.1']);
    console.log(datos['10.20.40.1']);
    /* Con este enfoque, hasta ahora, implica que para que se agregue un sitio habrá que actualizar página */
    Object.keys(marcas).forEach(function(est){
        if (datos[est].estado !== data[est].estado){
            console.log(data[est].estado, datos[est].estado, datos[est].hostname);
            marcas[est].setIcon(configuraIcono(datos[est]));
            marcas[est]._popup._content = creaTituloMarca(datos[est]);
        }
    }); 
   
    /* TODO: ¿Existe una mejor forma para agregar datos? ¿Es este método minímamente correcto */
    data = datos; 
};

/* Obtenemos los datos requeridos por primera vez*/
obtenerDatos().then(iniciaMarcadores).catch(function(e){
        console.log("NF: Hubo un error, puedo manejarlo");
        console.log(e);
    });


/* Obtenemos los datos requeridos para verificar su actualización cada cierto tiempo */
setInterval(function(){
    obtenerDatos().then(actualizaMarcadores).catch(function(e){
           console.log("NF: Hubo un error, puedo manejarlo");
           console.log(e);
       });

}, tiempoRecarga);


