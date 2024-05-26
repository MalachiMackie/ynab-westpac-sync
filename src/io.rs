use std::io::{stdin, stdout, Write};

use tokio::task::spawn_blocking;

pub async fn flush_stdout() -> anyhow::Result<()> {
    Ok(spawn_blocking(|| stdout().flush()).await??)
}

pub async fn read_u32() -> anyhow::Result<u32> {
    // use blocking because tokio docs recommend blocking calls for any user interactions. https://docs.rs/tokio/latest/tokio/io/fn.stdin.html
    spawn_blocking(|| {
        let mut buf = String::new();

        stdin().read_line(&mut buf)?;

        Ok(buf.trim().parse::<u32>()?)
    })
    .await?
}
