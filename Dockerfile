FROM amd64/rust:1-slim-bullseye as build

RUN apt-get update && apt-get install -y curl wget unzip

# Install protoc
RUN curl -s https://api.github.com/repos/protocolbuffers/protobuf/releases/latest \ 
          | grep "browser_download_url" \
          | grep "protoc-.*-linux-x86_64" \
          | cut -d : -f 2,3 \
          | tr -d '"' \
          | xargs wget -O ./protoc.zip \
    && unzip protoc.zip \
    && mv ./include/* /usr/include/ \
    && mv ./bin/protoc /usr/bin/protoc

# Install protoc-gen-openapiv2
RUN curl -s https://api.github.com/repos/grpc-ecosystem/grpc-gateway/releases/latest \
          | grep "browser_download_url" \
          | grep "protoc-gen-openapiv2-.*-linux-x86_64" \
          | cut -d : -f 2,3 \
          | tr -d '"' \
          | xargs wget -O ./protoc-gen-openapiv2 \
        && chmod +x protoc-gen-openapiv2 \
        && mv ./protoc-gen-openapiv2 /usr/bin/protoc-gen-openapiv2

# create a new empty shell project
RUN cargo new --bin visualizer-server
WORKDIR /visualizer-server

# sorry I'm too lazy to fix this right now
# # copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # this build step will cache your dependencies
# RUN cargo build --release \
#     && rm -rf ./src

# copy your source tree
COPY ./ ./

# build for release
RUN cargo build --release

# The final base image
FROM node:16-bullseye-slim

WORKDIR /usr/src/

# sol2uml needed phantom which installation needed bzip2
RUN apt-get update && apt-get install bzip2 \
    && npm install phantom \
    && npm link sol2uml@2.1 --only=production

# Copy from the previous build
COPY --from=build /visualizer-server/target/release/visualizer-server /usr/src/visualizer-server

# Run the binary
ENTRYPOINT ["/usr/src/visualizer-server"]
