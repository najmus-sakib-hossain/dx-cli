use dx_cli::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
