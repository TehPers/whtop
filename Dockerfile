# Build frontend
FROM rust:latest AS build-frontend

# Download build dependencies
RUN rustup self update && rustup update
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Copy sources into build folder
WORKDIR /build
COPY ./frontend ./frontend
COPY ./shared ./shared

# Build the application
WORKDIR /build/frontend/whtop_web
RUN trunk build --release --public-url /assets/ --dist /build/dist

# Build backend
FROM rust:latest AS build-backend

# Download build dependencies
RUN rustup self update && rustup update
RUN rustup target add x86_64-unknown-linux-musl

# Copy sources into build folder
WORKDIR /build
COPY ./backend ./backend
COPY ./shared ./shared

# Build the application
WORKDIR /build/backend/whtop
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN cp target/x86_64-unknown-linux-musl/release/whtop /build/whtop

# Deploy application
FROM scratch AS deploy

# Copy the outputs
WORKDIR /deploy
COPY --from=build-frontend /build/dist ./dist
COPY --from=build-backend /build/whtop ./whtop

# Run the application
ENV RUST_LOG="info" WHTOP_ADDRESS="0.0.0.0:8080"
ENTRYPOINT ["./whtop"]
