#! /bin/bash

#set -e
PWD=`pwd`
CONTAINER_NAME="api_gateway_microservice"
VERSION="0.1" 
PORT=9000
GRPC_PORT=5000
HOST_GRPC_PORT_MAP=5001
docker ps | grep -q "$CONTAINER_NAME"
if (( $? == 0 )); then
    echo "container already running"
else
    echo "container not running"
    CONTAINER_ID=$(docker run -d  -v `pwd`:`pwd` -p ${PORT}:${PORT} -p ${HOST_GRPC_PORT_MAP}:${GRPC_PORT}  ${CONTAINER_NAME}:${VERSION} /bin/bash -c "while true; do sleep 1000; done")

    docker exec -it "${CONTAINER_ID}" /bin/bash 
fi