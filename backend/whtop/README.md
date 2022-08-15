# whtop

The `whtop` server. This service provides endpoints for viewing metrics about the hardware that it's running on:

- `/cpu`: CPU metrics, including usage and frequency.
- `/memory`: RAM metrics, including total memory and used memory.
- `/processes`: Process metrics, including CPU usage and memory usage per process.

## Building

To build this service, use `cargo build`. To build for a release, use `cargo build --release`.

## Configuration

This service can be configured via environment variables:

- `RUST_LOG`: Configures the log level for the service. See the docs for [`tracing_subscriber::EnvFilter`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives) for more information.
- `WHTOP_ADDRESS`: The server address, including the port. For example: `0.0.0.0:8081`.
- `WHTOP_REFRESH_RATE_SECS`: The system info refresh rate. This is the minimum delay between system info updates.
