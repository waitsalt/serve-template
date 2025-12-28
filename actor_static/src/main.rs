use actor_static::util::log_init;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    log_init();

    Ok(())
}
