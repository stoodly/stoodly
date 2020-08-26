FROM rust:latest
WORKDIR /usr/src/stoodly
COPY . .
EXPOSE 8080
RUN cargo install --path .
CMD ["stoodly"]