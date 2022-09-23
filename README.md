# status-map
Un sencillo script para verificar el estado de dispositivos en base a los datos de LibreNMS

## Instrucciones para pruebas y desarrollo
Corremos en una terminal la aplicación
```bash 
cargo run
```

Y en otra trabajamos con el frontend:
```bash
docker run --name servidor -v $PWD/docker/default.conf:/etc/nginx/conf.d/default.conf:ro -v $PWD/public:/usr/share/nginx/html/ --rm -it -p 8000:80 nginx
```
