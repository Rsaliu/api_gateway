#! /bin/bash

set -e
CONTAINER_NAME="api_gateway_microservice"
VERSION="0.1" 
docker  build --build-arg USER_NAME=$(whoami) --build-arg USER_ID=$(id -u) --build-arg GROUP_ID=$(id -g) -t "${CONTAINER_NAME}":"${VERSION}" .