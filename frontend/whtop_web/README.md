# whtop_web

A web frontend for the whtop server.

## Dependencies

To build and run the frontend, you will need to install:

- The `wasm32-unknown-unknown` target:
  
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- `trunk`:

  ```sh
  cargo install trunk
  ```

## Building

You can use `trunk build` to build the frontend. To build for a release, use `trunk build --release`.

## Local development

The frontend can be tested locally by running:

```sh
trunk serve
```

`trunk` will monitor the code and automatically rebuild the frontend whenever a change is detected.
