use std::fs::File;

use greycat::prelude::*;

use crate::gc;

#[greycat_type]
pub struct CsvReader {
    records: Option<Box<csv::StringRecordsIntoIter<File>>>,
}

impl Object for CsvReader {
    fn finalize(&mut self, _ctx: GcMachine) {
        self.records.take();
    }
}

impl CsvReader {
    fn initialize_reader(&mut self, ctx: GcMachine) -> GcResult<()> {
        let path: GcString = unsafe { self.get_at(gc::csv2_CsvReader_path, ctx) };
        let options: Option<GcObject> = unsafe { self.get_at(gc::csv2_CsvReader_options, ctx) };
        
        let file = File::open(path.as_str())?;
        let mut builder = match options {
            Some(opts) => opts.reader_builder(ctx),
            None => csv::ReaderBuilder::new(),
        };
        let reader = builder.buffer_capacity(8192).from_reader(file);
        self.records.replace(Box::new(reader.into_records()));
        Ok(())
    }
}

#[greycat_impl]
impl CsvReader {
    pub fn can_read(&mut self, ctx: GcMachine) -> GcResult<bool> {
        let records = match self.records.as_mut() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_mut().unwrap()
            }
        };
        Ok(!records.reader().is_done())
    }

    pub fn read(&mut self, ctx: GcMachine) -> GcResult<Option<GcObject>> {
        let records = match self.records.as_mut() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_mut().unwrap()
            }
        };

        match records.next() {
            Some(record) => {
                let record = record?;
                let mut row = GcArray::new(ctx);
                for field in record.iter() {
                    if let Ok(value) = field.parse::<i64>() {
                        row.add(value, ctx);
                    } else if let Ok(value) = field.parse::<f64>() {
                        row.add(value, ctx);
                    } else if let Ok(value) = field.parse::<bool>() {
                        row.add(value, ctx);
                    } else if field.is_empty() {
                        row.add((), ctx);
                    } else {
                        row.add(field, ctx);
                    }
                }
                Ok(Some(row.as_object()))
            }
            None => Ok(None),
        }
    }
}

trait CsvOptions {
    fn reader_builder(&self, ctx: GcMachine) -> csv::ReaderBuilder;

    fn has_header(&self, ctx: GcMachine) -> bool;
    fn delimiter(&self, ctx: GcMachine) -> u8;
    fn flexible(&self, ctx: GcMachine) -> bool;
    fn trim(&self, ctx: GcMachine) -> Option<csv::Trim>;
}

impl CsvOptions for GcObject {
    fn reader_builder(&self, ctx: GcMachine) -> csv::ReaderBuilder {
        let mut builder = csv::ReaderBuilder::new();
        builder
            .has_headers(self.has_header(ctx))
            .delimiter(self.delimiter(ctx))
            .flexible(self.flexible(ctx));
        if let Some(trim) = self.trim(ctx) {
            builder.trim(trim);
        }
        builder
    }

    fn has_header(&self, ctx: GcMachine) -> bool {
        let value: Option<bool> = unsafe { self.get_at(gc::csv2_CsvOptions_has_header, ctx) };
        value.unwrap_or(true)
    }

    fn delimiter(&self, ctx: GcMachine) -> u8 {
        let value: Option<char> = unsafe { self.get_at(gc::csv2_CsvOptions_delimiter, ctx) };
        match value {
            Some(value) => value as u8,
            None => b',',
        }
    }

    fn flexible(&self, ctx: GcMachine) -> bool {
        let value: Option<bool> = unsafe { self.get_at(gc::csv2_CsvOptions_flexible, ctx) };
        value.unwrap_or_default()
    }

    fn trim(&self, ctx: GcMachine) -> Option<csv::Trim> {
        let value: Option<GcEnum> = unsafe { self.get_at(gc::csv2_CsvOptions_trim, ctx) };
        match value {
            Some(value) => match value.0.1 {
                gc::csv2_CsvTrim_All => Some(csv::Trim::All),
                gc::csv2_CsvTrim_Fields => Some(csv::Trim::Fields),
                gc::csv2_CsvTrim_Headers => Some(csv::Trim::Headers),
                _ => {
                    unreachable!("csv2 should take into account new CsvTrim fields")
                }
            },
            None => None,
        }
    }
}
