FROM public.ecr.aws/docker/library/rust:trixie AS builder
WORKDIR /src
# Add build dependencies
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update
RUN apt install -y musl-tools



COPY src ./src
COPY Cargo.toml .
RUN cargo build --bin rabbitmq --target x86_64-unknown-linux-musl --release

FROM scratch
WORKDIR /app
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/rabbitmq .
USER 1000
CMD [ "./rabbitmq" ]

