# Dockerfile for building rust applications

# to troubleshoot this build do:
#  DOCKER_BUILDKIT=0 docker build .

# Define a build-time variable with the ARG instruction
ARG IMAGE_TAG=3.19

FROM alpine:${IMAGE_TAG} AS build

RUN apk add rust cargo jq

WORKDIR /app

COPY . .

RUN cargo build --release

RUN APP_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].name') &&\
    echo "APP_NAME: ${APP_NAME}" &&\
    TARGET_DIRECTORY=$(cargo metadata --no-deps --format-version 1 | jq -r '.target_directory') &&\
    echo "TARGET_DIRECTORY: ${TARGET_DIRECTORY}" &&\
    cp ${TARGET_DIRECTORY}/release/${APP_NAME} /app/my_service

# https://docs.docker.com/build/building/multi-stage/

FROM alpine:${IMAGE_TAG}

RUN apk update && \
    apk add libgcc

COPY --from=build /app/my_service /bin/my_service

# Create a new user and group
RUN addgroup -g 1000 myappgroup && \
    adduser -u 1000 -G myappgroup -s /bin/sh -D myappuser

# Switch to the non-root user
USER myappuser

CMD [ "/bin/my_service" ]