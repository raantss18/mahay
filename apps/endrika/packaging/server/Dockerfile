# figcraft-server in a container; setup in docs/server.md ("Deploy on a Linux server"). Build from
# the repository root:
#
#   docker build -f packaging/server/Dockerfile -t figcraft-server .
#
# The image runs as an unprivileged user, keeps its files in the /data volume and reads
# /etc/figcraft-server/server.toml (listen on 0.0.0.0:47900, data in /data; mount your own to set
# public_url). `docker stop` sends SIGTERM: the server saves open files and stops gracefully.

FROM rust:1-bookworm AS build
WORKDIR /src
COPY . .
RUN cargo build --release --locked -p figcraft-server \
 && mkdir -p /out/data \
 && printf 'listen = "0.0.0.0:47900"\ndata = "/data"\n' > /out/server.toml

# glibc and libgcc only, no shell; "nonroot" is uid 65532.
FROM gcr.io/distroless/cc-debian12:nonroot
COPY --from=build /src/target/release/figcraft-server /usr/local/bin/figcraft-server
COPY --from=build /out/server.toml /etc/figcraft-server/server.toml
COPY --from=build --chown=nonroot:nonroot /out/data /data
USER nonroot
VOLUME /data
EXPOSE 47900
STOPSIGNAL SIGTERM
ENTRYPOINT ["/usr/local/bin/figcraft-server", "--config", "/etc/figcraft-server/server.toml"]
