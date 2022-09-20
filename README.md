# status-map
Un sencillo script para verificar el estado de dispositivos en base a los datos de LibreNMS

# 
run --name servidor -v $PWD/docker/default.conf:/etc/nginx/conf.d/default.conf:ro -v $PWD/public:/usr/share/nginx/html/ --rm -it -p 8000:80 nginx
