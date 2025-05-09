FROM ubuntu:22.04

# Install Ubuntu developer dependencies: we need
# - GTK and WebKit for Linux Tauri support

RUN apt-get update && apt-get install -y \
    curl locales ca-certificates  \
    build-essential scons git sudo pkg-config xdg-utils squashfs-tools patchelf \
    libssl-dev libcairo-dev libatk1.0-dev libsoup-3.0-dev librsvg2-dev \
    libjavascriptcoregtk-4.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev \
    libgl1-mesa-glx libglu1-mesa libcanberra-gtk-module libcanberra-gtk3-module \
    webkit2gtk-driver jq gh \
    libfuse2 xz-utils zsync desktop-file-utils glib2.0-bin file \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the locale
ENV LANG en_US.UTF-8
RUN locale-gen en_US.UTF-8 && update-locale LANG=en_US.UTF-8

# Install Node.js LTS and pnpm and tauri CLI
# WARNING: if you change the version, be sure to also change it on the GitHub Actions
# workflow file .github/workflows/build.yml
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    npm i pnpm @tauri-apps/cli --global

# Install the latest emsdk required to build stellarium-web-engine
RUN git clone https://github.com/emscripten-core/emsdk.git /opt/emsdk && \
    cd /opt/emsdk && \
    ./emsdk install latest && \
    ./emsdk activate latest && \
    echo "source /opt/emsdk/emsdk_env.sh" >> /etc/profile.d/emsdk.sh

# Install known Rust toolchain
RUN mkdir -p /opt/rust
ENV RUSTUP_HOME=/opt/rust
ENV PATH=/opt/rust/bin:$PATH
# WARNING: if you change the version, be sure to also change it on the GitHub Actions
# workflow file .github/workflows/build.yml
ENV RUST_VERSION=1.86.0

RUN export CARGO_HOME=/opt/rust && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --no-modify-path --default-toolchain $RUST_VERSION -y --profile minimal \
    -c rustfmt -c clippy -c rust-src -c llvm-tools -c rust-analyzer

# Install cargo tauri tool
RUN \
    cargo install tauri-cli --version '^2.0.0' --locked && \
    mv /root/.cargo/bin/cargo-tauri /usr/local/bin && \
    rm -Rf /root/.cargo

# Add a vscode user with sudo privileges
RUN groupadd -g 1000 vscode && \
    useradd -m -u 1000 -g vscode -s /bin/bash vscode && \
    echo 'vscode ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers.d/vscode && \
    chmod 0440 /etc/sudoers.d/vscode

COPY rustup_wrapper.sh /usr/local/bin/rustup
COPY rust.sh /etc/profile.d/rust.sh

WORKDIR /workspace

# Set ownership of the workspace directory to the vscode user
RUN chown -R vscode:vscode /workspace

CMD ["bash"]
