ES_CLIENT_PASSWORD="elastic"

docker network create elasticsearch
docker pull elasticsearch:8.11.2
sudo sysctl -w vm.max_map_count=262144
docker compose up -d
