FROM amd64/rust:1-slim-bullseye as build

# create a new empty shell project
RUN cargo new --bin sol_to_uml
WORKDIR /sol_to_uml

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm -rf ./src

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release

# The final base image
FROM node:16-bullseye-slim

RUN apt-get update

WORKDIR /usr/src/

# sol2uml needed phantom which installation needed bzip2
RUN apt install bzip2
RUN npm install phantom
RUN npm link sol2uml --only=production

# Copy from the previous build
COPY --from=build /sol_to_uml/target/release/sol_to_uml /usr/src/sol_to_uml

# Run the binary
CMD ["/usr/src/sol_to_uml"]
