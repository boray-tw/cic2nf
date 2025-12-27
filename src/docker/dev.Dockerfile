FROM rust:1.92-slim-bookworm

# reference: https://docs.docker.com/build/building/best-practices/#apt-get
RUN apt-get update -qqy \
  && apt-get install -qqy gosu \
  && rm -rf /var/lib/apt/lists/*

# ensure we execute commands in a Docker container
# ref: https://stackoverflow.com/a/65942222/27092911
ENV IS_DOCKER_RUNNING=true

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
