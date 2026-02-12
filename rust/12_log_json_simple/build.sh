#!/bin/bash

NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].name')
VERSION=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version')
CONTAINER_REPO="192.168.1.102:5000"

echo "docker build --tag ${NAME}:${VERSION} ." && docker build --tag ${NAME}:${VERSION} .
if [ $? -ne 0 ]; then
    echo "Docker build failed"
    exit 1
fi
echo "docker tag ${NAME}:${VERSION} ${CONTAINER_REPO}/${NAME}:${VERSION}" && docker tag ${NAME}:${VERSION} ${CONTAINER_REPO}/${NAME}:${VERSION}
if [ $? -ne 0 ]; then
    echo "Docker tag failed"
    exit 2
fi
echo "docker push ${CONTAINER_REPO}/${NAME}:${VERSION}" && docker push ${CONTAINER_REPO}/${NAME}:${VERSION}
if [ $? -ne 0 ]; then
    echo "Docker push failed"
    exit 3
fi
echo "curl https://${CONTAINER_REPO}/v2/_catalog" && curl https://${CONTAINER_REPO}/v2/_catalog
if [ $? -ne 0 ]; then
    curl --cacert ~/tmp/k8s_ca.crt  https://${CONTAINER_REPO}/v2/_catalog
    if [ $? -ne 0 ]; then
      echo "repo catalog request failed"
      exit 4
    fi
fi


echo "kubectl create deployment ${NAME} --image=${CONTAINER_REPO}/${NAME}:${VERSION} --replicas=2"