mod blocks;
mod blockchain;
mod errors;
mod cli;
mod transaction;
use crate::errors::Result;
use crate::cli::Cli;
fn main() -> Result<()> {
    println!("hello world");
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())

}
