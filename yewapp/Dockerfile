FROM rust:1.49 as builder

RUN USER=root cargo new --bin rust-docker-yew
WORKDIR /rust-docker-yew
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/yewapp*
RUN trunk build --release 

FROM nginx:1.19.6-alpine

COPY --from=builder /rust-docker-yew/dist/* /usr/share/nginx/html/
