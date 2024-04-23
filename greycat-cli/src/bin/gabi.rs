use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use greycat::prelude::*;

#[derive(Parser)]
struct Args {
    #[arg(long, help = "The path to the ABI", default_value = "gcdata/store/abi")]
    abi: PathBuf,

    #[arg(help = "The file to read")]
    filepath: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let abi_file = File::open(&args.abi).context("unable to read abi file")?;
    let abi_buf = BufReader::new(abi_file);
    let abi = Abi::new(abi_buf, None)?;

    let value_buf = std::fs::read(&args.filepath).context("unable to read value file")?;
    let mut value_bytes = &value_buf[..];

    let v_headers = value_bytes.read_request_headers()?;
    if abi.headers.headers.protocol != v_headers.protocol {
        anyhow::bail!(
            "mismatched ABI protocol (got={}, expected={})",
            v_headers.protocol,
            abi.headers.headers.protocol
        );
    }
    let mut bytes = value_bytes;
    loop {
        if bytes.is_empty() {
            break;
        }

        let value = bytes.read_value(&abi)?;
        println!("{value:#?}");
    }

    Ok(())
}
