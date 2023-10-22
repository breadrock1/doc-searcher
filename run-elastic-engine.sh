ES_CLIENT_PASSWORD="elastic"

docker network create elastic
docker pull docker.elastic.co/elasticsearch/elasticsearch:8.10.2
sudo sysctl -w vm.max_map_count=262144
docker run --rm --name es01 --net elastic \
  -m 1GB \
  -p 9200:9200 \
  -e ELASTIC_PASSWORD=$ES_CLIENT_PASSWORD \
  -it docker.elastic.co/elasticsearch/elasticsearch:8.10.2

docker cp es01:/usr/share/elasticsearch/config/certs/http_ca.crt .
