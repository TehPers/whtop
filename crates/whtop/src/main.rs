mod config;
mod errors;
mod routes;
mod startup;
mod layers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    startup::start().await
}
