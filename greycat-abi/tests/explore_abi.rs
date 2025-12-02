use greycat_abi::{Abi, FromVec};

mod utils;

#[test]
fn list_symbols() -> anyhow::Result<()> {
    let abi = Abi::from_vec(std::fs::read(utils::test_resource("abi"))?)?;
    assert_eq!(abi.symbols.into_iter().count(), 1583);
    Ok(())
}

#[test]
fn get_symbol() -> anyhow::Result<()> {
    let abi = Abi::from_vec(std::fs::read(utils::test_resource("abi"))?)?;
    assert_eq!(&abi.symbols[42], "thousands_separator");
    Ok(())
}

#[test]
fn has_symbol() -> anyhow::Result<()> {
    let abi = Abi::from_vec(std::fs::read(utils::test_resource("abi"))?)?;
    assert_eq!(&abi.symbols[42], "thousands_separator");
    Ok(())
}
