let  habajo = require("./../iconos/h/abajo.png")
let  harriba = require("./../iconos/h/arriba.png")
let  hdeshabilitado = require("./../iconos/h/deshabilitado.png")
let  hadvertencia = require("./../iconos/h/advertencia.png")

let posibilidaes = {
    hospital: {
        arriba: harriba,
        abajo: habajo,
        deshabilitado: hdeshabilitado,
        advertencia: hadvertencia
    }
}

// TODO: Revisar si la opción no existe, al menos para que no tiré error
let configurarIcono = (tipo, estado) => {
    let iconUrl = posibilidaes[tipo][estado];
    // TODO: Hay más opciones interesantes a revisar
    return L.icon({iconUrl, iconSize: [16, 16]}); 
}

export {configurarIcono};