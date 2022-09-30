# status-map
Un sencillo script para verificar el estado de dispositivos en base a los datos de LibreNMS

## Instrucciones para pruebas y desarrollo
Construimos el frontend:
```bash
yarn build
```
Corremos en una terminal la aplicación
```bash 
cargo run
```

Y en otra trabajamos con el frontend:
```bash
podman run --name servidor -v $PWD/docker/default.conf:/etc/nginx/conf.d/default.conf:ro -v $PWD/public:/usr/share/nginx/html/ --rm -it -p 8000:80 nginx
```

Por ahora, y entre otras cosas porque todavía no reviso lo del POST, y también porque sale más cómodo así que crear la petición con JSON, probaré así: 
```bash
curl -v localhost:8080/cartero/81410673-2d6d-48cb-b4ea-c48388a6f23f/1663960567
```