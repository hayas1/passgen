# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.166.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:0-1

# use npm not apt-get but n package
# RUN apt-get update
# RUN apt-get -y install --no-install-recommends nodejs npm
# RUN npm install n -g && n stable
# RUN apt-get purge -y nodejs npm


# RUN rustup install nightly \
#     && rustup default nightly
RUN rustup target add wasm32-unknown-unknown \
    && cargo install wasm-pack \
    && cargo install miniserve
