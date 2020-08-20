FROM fedora:32
RUN dnf install -y openssl-devel rust cargo
COPY . /app/
WORKDIR /app
RUN cargo build


FROM fedora:32
MAINTAINER Michael Scherer <misc@redhat.com>
COPY --from=0 /app/target/debug/well-actually-bot .
CMD ["./well-actually-bot"]
