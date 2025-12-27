#!/bin/bash
# config
IMAGE_NAME="pcap-to-nf"
TO_REBUILD_IMAGE=1 # 0 for no; 1 for yes
INPUT_DIR="$(pwd)/input"
OUTPUT_DIR="$(pwd)/output"
CWD_CONTAINER=/pcap-to-nf

has_image() {
  if [ -z "$(docker images -q ${IMAGE_NAME})" ]; then
    return 1 # false
  fi
  return 0 # true
}

if [ ! -z "$(docker container ls --format "{{.Names}}" | grep "^${IMAGE_NAME}$" 2> /dev/null)" ]; then
  docker stop "${IMAGE_NAME}"
  sleep 0.5
fi

if [ ! -z "$(docker container ls --all --format "{{.Names}}" | grep "^${IMAGE_NAME}$" 2> /dev/null)" ]; then
  docker rm "${IMAGE_NAME}"
  sleep 0.5
fi

if [ "${TO_REBUILD_IMAGE}" != 0 ] && has_image; then
  docker rmi "${IMAGE_NAME}"
fi

if ! has_image; then
  docker build --tag "${IMAGE_NAME}:latest" .
fi

mkdir -p "${INPUT_DIR}" "${OUTPUT_DIR}"

docker run --rm -it --name "${IMAGE_NAME}" --hostname "${IMAGE_NAME}" \
  --mount type=bind,src="${INPUT_DIR}",target="${CWD_CONTAINER}"/input \
  --mount type=bind,src="${OUTPUT_DIR}",target="${CWD_CONTAINER}"/output \
  "${IMAGE_NAME}:latest"
