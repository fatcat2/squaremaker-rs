FROM ubuntu:24.04

# Setup
RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl \
    graphicsmagick \
    libgraphicsmagick1-dev \
    llvm-dev \
    libclang-dev \
    clang
RUN apt-get update

# Install Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/squaremaker-rs"]
