use anyhow::{Result, anyhow};
use std::env;
use std::process::exit;

fn test(value: &str) -> Result<()> {
    println!("INFO: value: {value:?}");

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let value = args
        .get(1)
        .ok_or("specify string")
        .map_err(|e| anyhow!(e))?;

    test(value)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = args
        .first()
        .ok_or("failed to get program name")
        .map_err(|e| anyhow!(e))?;

    let result = real_main();

    if let Err(e) = result {
        eprintln!("ERROR: {program_name}: {e:#?}");
        exit(1);
    }

    result
}
