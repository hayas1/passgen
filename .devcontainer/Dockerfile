# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.166.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# [Optional] Uncomment this section to install additional packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# RUN rustup install nightly
# RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack
RUN cargo install miniserve
