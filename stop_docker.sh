#! /bin/bash

set -e

CONTAINER_NAME="api_gateway_microservice"

docker ps | grep "${CONTAINER_NAME}" | awk '{printf "%s\n", $1}'| xargs -I {} docker stop {}