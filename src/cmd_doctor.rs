use anyhow::Result;

use crate::config::Config;
use crate::llm;

pub async fn run() -> Result<()> {
    for (k, v) in Config::summarize() {
        println!("{k:<12} {v}");
    }
    match Config::load(None) {
        Ok(cfg) => match llm::list_models(&cfg).await {
            Ok(models) => println!("connection  ok, {} models available.", models.len()),
            Err(e) => println!("connection  failed: {e:#}"),
        },
        Err(_) => println!("connection  skipped: API key not set."),
    }
    Ok(())
}

pub async fn models() -> Result<()> {
    let cfg = Config::load(None)?;
    let ids = llm::list_models(&cfg).await?;
    for id in &ids {
        if *id == cfg.model {
            println!("{id}  <- current");
        } else {
            println!("{id}");
        }
    }
    Ok(())
}
