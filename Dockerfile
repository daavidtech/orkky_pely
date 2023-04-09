FROM rust:1.67

RUN apt-get update
RUN apt-get install -y alsa-tools librust-alsa-sys-dev libudev-dev

RUN cargo install wasm-pack

WORKDIR /usr/src/orkky_pely
COPY . .

RUN cargo build

RUN wasm-pack build --target web
