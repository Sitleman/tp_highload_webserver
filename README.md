#### Start rust webserver:
<code>
docker build -t rustws -f Dockerfile .

docker run -p 8081:8081 -v /home/nikita/rust/httptest:/httptest:ro --name rustws_container -t rustws
</code>

#### Start nginx webserver:
<code>
docker build -t nginxws -f Dockerfile.nginx .

docker run -p 8082:80 -v /home/nikita/rust/httptest:/var/www/html/httptest:ro --name nginxws_container -t nginxws
</code>
