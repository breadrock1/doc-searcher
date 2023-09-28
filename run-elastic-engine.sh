docker network create elastic
docker pull docker.elastic.co/elasticsearch/elasticsearch:8.10.2
docker run --name es01 --net elastic -p 9200:9200 -it -m 1GB docker.elastic.co/elasticsearch/elasticsearch:8.10.2

docker exec -it es01 /usr/share/elasticsearch/bin/elasticsearch-reset-password -u elastic -i "vnw5tiRVSKYyNVpbkhEq"
docker exec -it es01 /usr/share/elasticsearch/bin/elasticsearch-create-enrollment-token -s kibana
export ELASTIC_PASSWORD="vnw5tiRVSKYyNVpbkhEq"

docker cp es01:/usr/share/elasticsearch/config/certs/http_ca.crt .
