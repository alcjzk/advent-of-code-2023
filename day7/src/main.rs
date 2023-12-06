use anyhow::Result;
use std::fs::OpenOptions;

fn main() -> Result<()> {
    let file = OpenOptions::new().read(true).open("test");
    Ok(())
}
