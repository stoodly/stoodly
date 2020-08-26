FROM rust:latest
WORKDIR /usr/src/stand-up
COPY . .
EXPOSE 8080
RUN cargo install --path .
CMD ["stand-up"]