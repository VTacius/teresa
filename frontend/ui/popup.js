let estadoServidor = (estado) => {
    return estado ? '<strong class="estado arriba">Arriba</strong>' : '<strong class="estado abajo">Abajo</strong>'
}

let contenidoServidores = (servidores) => servidores.map((servidor) => {
    return `<li class="servidor">
      <span class="nombre">${servidor.tipo}</span>
      <span class="datos caja">${estadoServidor(servidor.estado)}</span>
      <span class="datos">${servidor.ttl}ms</span>
      <span class="datos">${servidor.rtt}</span>
    </li>
    `
});

let contenidoEstablecimiento = (nombre, servidores) => {
    return `
        <div class="info">
        <article>
          <header>
            <h1>
            ${nombre}
            </h1>
          </header>
          <ul>
            ${contenidoServidores(servidores).join("")}
          </ul>
        </article>
        </div>
        `
}

export { estadoServidor, contenidoServidores, contenidoEstablecimiento};