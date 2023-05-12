FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

ADD . /xxhash-rust/

WORKDIR /xxhash-rust/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly


COPY --from=builder /xxhash-rust/fuzz/target/x86_64-unknown-linux-gnu/release/xxh64 /

COPY --from=builder /xxhash-rust/fuzz/target/x86_64-unknown-linux-gnu/release/xxh32 /

COPY --from=builder /xxhash-rust/fuzz/target/x86_64-unknown-linux-gnu/release/xxh3_secret /

COPY --from=builder /xxhash-rust/fuzz/target/x86_64-unknown-linux-gnu/release/xxh3_seed /

COPY --from=builder /xxhash-rust/fuzz/target/x86_64-unknown-linux-gnu/release/xxh3 /