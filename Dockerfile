# stage 1 - generaete recipe file
FROM rust as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . . 
RUN cargo chef prepare --recipe-path recipe.json

# stage 2 - build our dependencies
FROM rust as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# stage 3 - Use main official Rust image
FROM rust as builder

# copy app into image
COPY . /app

# set work directory
WORKDIR /app

# Copy dependencies 
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# build the app
RUN cargo build --release

# new running image from google distroless
FROM gcr.io/distroless/cc-debian11

# copy app from builder
COPY --from=builder /app/target/release/s-rust-plegium-webhook /app/s-rust-plegium-webhook
WORKDIR /app

# start the application
CMD ["./s-rust-plegium-webhook"]
