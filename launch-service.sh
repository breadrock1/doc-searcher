ES_CLIENT_PASSWORD="elastic"

docker network create elastic
docker pull docker.elastic.co/elasticsearch/elasticsearch:8.10.2
sudo sysctl -w vm.max_map_count=262144
docker compose up -d
