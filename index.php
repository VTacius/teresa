<html>
<head>
    <title>Prueba de mapa</title>
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.2.0/dist/leaflet.css"
        integrity="sha512-M2wvCLH6DSRazYeZRIm1JnYyh22purTM+FDB5CsyxtQJYeKq83arPe5wgbNmcFXGqiSH2XR8dT/fJISVA1r/zQ=="
        crossorigin=""/>
    <style> 
        #mapid { 
            height: 99%; 
        }
    </style>
</head>
<body>
    <div id="mapid"></div>
</body>
<script src="https://unpkg.com/leaflet@1.2.0/dist/leaflet.js"
    integrity="sha512-lInM/apFSqyy1o6s89K4iQUKg6ppXEgsVxT35HbzUupEVRh2Eu9Wdl4tHj7dZO0s1uvplcYGmt3498TtHq+log=="
    crossorigin=""></script>
<script>
    var api = '/estado.php';
	var datos = {};

    var peticion = new XMLHttpRequest();
    peticion.open('GET', api, true);

    peticion.onreadystatechange = function (){
   		if (this.readyState === 4) {
    		if (this.status >= 200 && this.status < 400) {
      			var datos = JSON.parse(this.responseText);
				iniciaMarcadores(datos);
    		} else {
      			console.log('Hubo un error al intentar acceder a los datos');
    		}
  		} 
    }

	peticion.send();
	peticion = null;

    /* Desde ya, empiezo a darle forma a los datos */
    /*var datos = {
        est1 : {
            nombre : "Establecimiento Uno",
            latitude: 13.5079200000,
            longitude: -88.8578400000,
            estado: 'down', 
            duracion: '2 días, 2 horas',
            tipo: 'hospital'
        }, 
        est2 : {
            nombre : "Establecimiento Dos: Tienen un nombre escasamente extenso para mostrar a los usuarios",
            latitude: 13.7058308540,
            longitude: -90.0222985180,
            estado: 'up' ,
            duracion: '3 días, 0 horas',
            tipo: 'hospital'
        },
        est3 : {
            nombre : "Establecimiento Tres: Un nombre bonito para un bonito lugar",
            latitude: 13.8374896689,
            longitude: -89.44257468,
            estado: 'up' ,
            duracion: '78 días, 5 horas',
            tipo: 'hospital'
        },
        est4 : {
            nombre : "Establecimiento Tres: Un nombre bonito para un bonito lugar",
            latitude: 14.3682306190,
            longitude: -89.0929615700,
            estado: 'up' ,
            duracion: '78 días, 5 horas',
            tipo: 'clinica'
        },
        est5 : {
            nombre : "Establecimiento Cinco: Un nombre bonito para un bonito lugar",
            latitude: 13.4169016590,
            longitude: -88.3438239160,
            estado: 'up' ,
            duracion: '78 días, 5 horas',
            tipo: 'clinica'
        }

    };*/
    
    /* Esta simboliza por ahora un pedido en la actualización de todos los datos */
    /*var datosActualizacion = {
        est1 : {
            nombre : "Establecimiento Uno",
            latitude: 13.5079200000,
            estado: 'up', 
            duracion: '2 horas',
            longitude: -88.8578400000,
            tipo: 'hospital'
        }, 
        est2 : {
            nombre : "Establecimiento Dos: Tienen un nombre escasamente extenso para mostrar a los usuarios",
            latitude: 13.7058308540,
            longitude: -90.0222985180,
            estado: 'up' ,
            duracion: '3 días, 0 horas',
            tipo: 'hospital'
        },
        est3 : {
            nombre : "Establecimiento Tres: Un nombre bonito para un bonito lugar",
            latitude: 13.8374896689,
            longitude: -89.44257468,
            estado: 'down' ,
            duracion: '3 minutos',
            tipo: 'hospital'
        },
        est4 : {
            nombre : "Establecimiento Tres: Un nombre bonito para un bonito lugar",
            latitude: 14.3682306190,
            longitude: -89.0929615700,
            estado: 'up' ,
            duracion: '78 días, 5 horas',
            tipo: 'clinica'
        },
        est5 : {
            nombre : "Establecimiento Cinco: Un nombre bonito para un bonito lugar",
            latitude: 13.4169016590,
            longitude: -88.3438239160,
            estado: 'down' ,
            duracion: '1 días, 5 horas',
            tipo: 'clinica'
        }

    };*/
    
    /* Pues supongo que así se harán las marcas */
    var marcas = {};

    /* Configurando algunos valores por defecto */
    var default_lat = "13.8054";
    var default_lng = "-88.9069";
    var default_zoom = 10;
    var tile_url = "{s}.tile.openstreetmap.org";

    /* Facilita la creación de un titulo para el popup de los marcadores */ 
    var creaTituloMarca = function(dataObjeto){
        return '<b>' + dataObjeto.nombre + '</b>:<br>' + 
            'Estado: <b>' + configuraEstado(dataObjeto.estado) + '</b>' +
            ' desde <b>' + dataObjeto.duracion + '</b>'
    };

    
    /* En este momento tan temprano, algo como los íconos son totalmente necesarios */
    var LeafIcon = L.Icon.extend({
        options: {
            iconSize: [16, 16],
        }
    });

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

    /* El ícono es bastante cambiante: Se basa en tipo de marcador y estado actual */
    var configuraIcono = function(dataObjeto){
        if (dataObjeto.tipo in iconos){
            rtipo = dataObjeto.tipo;
        } else {
            /* TODO: Establecer un tipo por defecto */
            rtipo = 'sibasi'; 
        }
        ricon = iconos[rtipo][configuraEstado(dataObjeto.estado)];
        return ricon;
    };

    /* Actualizamos sólo aquellas marcas que de veras lo requieran */
    /*
    var actualizaDatos = function(){
        /* Con este enfoque, hasta ahora, implica que para que se agregue un sitio habrá que actualizar página 
        Object.keys(marcas).forEach(function(est){
            if (datos[est].estado !== datosActualizacion[est].estado){
				var estado = configuraEstado(datosActualizacion[est]['estado']);
                marcas[est].setIcon(configuraIcono(estado));
                marcas[est]._popup._content = creaTituloMarca(datosActualizacion[est]);
            }
        }); 
    };*/

	/* Una vez el servidor nos envía un datos totalmente coherente, es acá donde habrá que darle un valor útil al usuario */
	var configuraEstado = function(estado){
		if (estado == 0){
			return 'down';
		}else if (estado == 1){
			return 'up';
		}else {
			return 'unk';
		}
	}

    /* Esto es básicamente la configuración inicial del mapa */
    var mymap = L.map('mapid').setView([default_lat, default_lng], default_zoom);
    L.tileLayer('//' + tile_url + '/{z}/{x}/{y}.png', {
        attribution: '<a href="http://osm.org/copyright">OpenStreetMap</a> contributors'
    }).addTo(mymap);

    /* Hacemos la creación inicial de los marcadores */
	var iniciaMarcadores = function(datos){
    	Object.keys(datos).forEach(function(clave){
    	    marcas[clave] = L.marker([datos[clave].latitude, datos[clave].longitude], {icon: configuraIcono(datos[clave])}).addTo(mymap);
    	    /* TODO: Debe ser una función bien bonita que incluso pudiera poner valores por defecto, y modificarlos después */
    	    marcas[clave].bindPopup(creaTituloMarca(datos[clave]));
    	});
	};

	/*
    setTimeout(function(){
        actualizaDatos();
    }, 4000);*/
</script>

</html>
