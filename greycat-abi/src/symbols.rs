use crate::deserialize::Deserialize;
use crate::Result;
use anyhow::Context;
use greycat::GcSymbolId;

#[derive(Default, Clone, PartialEq)]
pub struct AbiSymbols {
    data: Box<str>,
    symbols: Box<[(u32, u32)]>,
}

impl AbiSymbols {
    #[inline]
    pub fn resolve(&self, id: GcSymbolId) -> &str {
        let (off, len) = self.symbols[id.0 as usize];
        &self.data[off as usize..off as usize + len as usize]
    }
}

impl Deserialize for AbiSymbols {
    fn deserialize<D>(de: &mut D) -> Result<Self>
    where
        D: crate::Deserializer,
    {
        let symbol_size = de.read_u64().context("abi symbols: size")?;
        let nb_symbols = de.read_u32().context("abi symbols: count")? as usize;

        let mut data = Vec::with_capacity(symbol_size as usize);
        let mut symbols = Vec::with_capacity(nb_symbols + 1);

        symbols.push((0, 0)); // the unknown symbol

        // TODO: add inverse map, to be able to resolve: string -> id

        for i in 0..nb_symbols {
            let off = data.len() as u32;
            let len = de
                .read_vu32()
                .with_context(|| format!("abi symbol {}: length", i + 1))?;
            data.resize(data.len() + len as usize, 0);
            de.read_into(&mut data[off as usize..])
                .with_context(|| format!("abi symbol {}: data", i + 1))?;
            symbols.push((off, len));
        }

        let data = unsafe { String::from_utf8_unchecked(data).into_boxed_str() };

        Ok(Self {
            data,
            symbols: symbols.into_boxed_slice(),
        })
    }
}

impl std::ops::Index<usize> for AbiSymbols {
    type Output = str;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.resolve(GcSymbolId(index as u32))
    }
}

impl std::ops::Index<u32> for AbiSymbols {
    type Output = str;

    #[inline(always)]
    fn index(&self, index: u32) -> &Self::Output {
        self.resolve(GcSymbolId(index))
    }
}

impl std::ops::Index<i32> for AbiSymbols {
    type Output = str;

    #[inline(always)]
    fn index(&self, index: i32) -> &Self::Output {
        self.resolve(GcSymbolId(index as u32))
    }
}

impl std::fmt::Debug for AbiSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbiSymbols")
            .field("symbols", &self.into_iter().collect::<Vec<_>>())
            .finish()
    }
}

impl<'a> IntoIterator for &'a AbiSymbols {
    type Item = &'a str;
    type IntoIter = AbiSymbolsIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        AbiSymbolsIter {
            symbols: self,
            index: 1,
        }
    }
}

#[derive(Debug)]
pub struct AbiSymbolsIter<'a> {
    symbols: &'a AbiSymbols,
    index: u32,
}

impl<'a> Iterator for AbiSymbolsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.symbols.symbols.len() as u32 {
            return None;
        }
        let symbol = self.symbols.resolve(GcSymbolId(self.index));
        self.index += 1;
        Some(symbol)
    }
}
