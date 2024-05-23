FROM rust:1.78.0-slim-buster AS base

# Set the working directory
WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

# Expose the port the application will run on
EXPOSE 3000

CMD [ "cargo", "run", "--offline" ]
