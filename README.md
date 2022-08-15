# whtop

A server monitoring tool.

## Dependencies

`whtop` only depends on `cargo`, which can be installed with [`rustup`](https://rustup.rs/).

`whtop_web` has the following additional dependencies:

- The `wasm32-unknown-unknown` target:
  
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- `trunk`:

  ```sh
  cargo install trunk
  ```

Finally, `cargo-make` is needed to run the tasks defined in `Makefile.toml`:

```sh
cargo install cargo-make
```

## Running locally

To run the frontend and backend locally, run:

```sh
cargo make run
```

The frontend will automatically rebuild whenever a change is detected. The backend will not rebuild
automatically, however.

### Running in Docker

First, build the image:

```sh
cargo make build-docker
```

Then, run the image:

```sh
cargo make run-docker
cargo make monitor-docker

# To stop the server:
cargo make stop-docker
```

There is also a `docker-compose.yaml` file included for convenience. To use it, run:

```sh
docker compose build
docker compose up
```

## License

This code is licensed under your choice of either [MIT](./LICENSE-MIT) or [Apache 2.0](./LICENSE-APACHE).
