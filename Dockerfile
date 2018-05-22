# Stage 0 - Build

FROM rust:1.26.0

RUN cargo install cargo-build-deps

RUN apt update; \
    apt install -y --no-install-recommends \
    make \
    libssl1.0-dev \
    pkg-config \
    libsodium-dev \
    libopus-dev \
    ffmpeg \
    youtube-dl \
    ;

RUN cd /opt && USER=root cargo new --bin app

WORKDIR /opt/app

COPY Cargo* ./

COPY lib lib/

RUN cargo build-deps --release

COPY src src

RUN cargo build --all --release

# Stage 1 - Copy only distributables

FROM debian:stable-slim

ENV LD_LIBRARY_PATH=/root/release/lib

RUN apt update && apt install -y --no-install-recommends \
    ca-certificates \
    libssl1.0-dev \
    libsodium-dev \
    libopus-dev \
    ffmpeg \
    youtube-dl \
    ;

WORKDIR /root/release

COPY --from=0 /usr/local/rustup/toolchains/1.26.0-x86_64-unknown-linux-gnu/lib/ lib/

COPY --from=0 /opt/app/target/release ./

RUN mkdir plugins && cp `ls | grep .so` ./plugins

CMD ["/root/release/void_game_bot_ws"]