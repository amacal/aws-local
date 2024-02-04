FROM mcr.microsoft.com/devcontainers/rust:bookworm
RUN apt update && apt install -y python3-pip
RUN pip3 install --break-system-packages cargo-lambda

USER vscode
RUN rustup update 1.75.0
RUN rustup default 1.75.0
RUN rustup component add rustfmt
RUN cargo install cargo-watch
