FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    curl \
    build-essential scons \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN curl -fsSL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y nodejs && \
    pnpm install -g pnpm@latest

RUN \
    cargo install tauri-cli --version '^2.0.0' --locked && \
    cargo install trunk --locked && \
    cargo install leptosfmt --locked && \
    rustup target add wasm32-unknown-unknown

WORKDIR /workspace

CMD ["bash"]