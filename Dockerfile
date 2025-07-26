FROM quay.io/fedora/fedora:latest
RUN dnf install -y rust cargo && dnf clean all
COPY . /app/
WORKDIR /app
RUN cargo build --release

FROM quay.io/fedora/fedora:latest
COPY --from=0 /app/target/release/well-actually-bot .
CMD ["./well-actually-bot"]
