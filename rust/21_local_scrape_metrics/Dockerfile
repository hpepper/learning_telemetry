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

#RUN export APP_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].name')

#RUN cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].name' > /app/app_name.txt
#RUN cargo metadata --no-deps --format-version 1 | jq -r '.target_directory' > /app/target_directory.txt
RUN APP_NAME=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].targets[].name') &&\
    echo "APP_NAME: ${APP_NAME}" &&\
    TARGET_DIRECTORY=$(cargo metadata --no-deps --format-version 1 | jq -r '.target_directory') &&\
    echo "TARGET_DIRECTORY: ${TARGET_DIRECTORY}" &&\
    cp ${TARGET_DIRECTORY}/release/${APP_NAME} /app/my_service

# https://docs.docker.com/build/building/multi-stage/

FROM alpine:${IMAGE_TAG}

RUN apk update && \
    apk add libgcc

#COPY --from=build /app/app_name.txt /tmp/app_name.txt
#RUN APP_NAME=$(cat /tmp/app_name.txt)
#RUN echo "APP_NAME: ${APP_NAME}"
#RUN cat /tmp/app_name.txt
#COPY --from=build /app/target_directory.txt /tmp/target_directory.txt
#RUN TARGET_DIRECTORY=$(cat /tmp/target_directory.txt)
#RUN echo "TARGET_DIRECTORY: ${TARGET_DIRECTORY}"

#COPY --from=build ${TARGET_DIRECTORY}/release/$APP_NAME /bin/hello
# INVALID: COPY --from=build $(cat /tmp/target_directory.txt)/release/$(cat /tmp/app_name.txt) /bin/hello
COPY --from=build /app/my_service /bin/my_service

# Create a new user and group
RUN addgroup -g 1000 myappgroup && \
    adduser -u 1000 -G myappgroup -s /bin/sh -D myappuser

# Switch to the non-root user
USER myappuser

CMD [ "/bin/my_service" ]