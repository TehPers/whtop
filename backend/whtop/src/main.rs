mod config;
mod errors;
mod layers;
mod routes;
mod startup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    startup::start().await
}
