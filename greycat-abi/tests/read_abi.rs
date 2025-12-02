mod utils;
use std::io::BufReader;

use greycat_abi::*;

#[test]
fn from_bytes() -> anyhow::Result<()> {
    let bytes = std::fs::read(utils::test_resource("abi"))?;
    let abi = Abi::from_bytes(bytes.as_slice())?;
    println!("{abi:#?}");
    Ok(())
}

#[test]
fn from_vec() -> anyhow::Result<()> {
    let bytes = std::fs::read(utils::test_resource("abi"))?;
    let abi = Abi::from_vec(bytes)?;
    println!("{abi:#?}");
    Ok(())
}

#[test]
fn from_reader() -> anyhow::Result<()> {
    let file = std::fs::File::open(utils::test_resource("abi"))?;
    let mut reader = BufReader::new(file);
    let abi = Abi::from_reader(&mut reader)?;
    println!("{abi:#?}");
    Ok(())
}
