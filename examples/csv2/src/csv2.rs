use std::fs::File;

use greycat::prelude::*;

type Result<T> = std::result::Result<T, std::io::Error>;

#[greycat_type]
pub struct CsvReader {
    records: Option<Box<csv::StringRecordsIntoIter<File>>>,
}

impl CsvReader {
    fn initialize_reader(&mut self, ctx: GcMachine) -> Result<()> {
        let file = File::open(self.path(ctx).as_str())?;
        let reader = csv::ReaderBuilder::new()
            // TODO map builder options to CsvFormat
            .has_headers(true)
            .flexible(true)
            .buffer_capacity(8192)
            .from_reader(file);
        self.records.replace(Box::new(reader.into_records()));
        Ok(())
    }
}

#[greycat_impl]
impl CsvReader {
    pub fn can_read(&mut self, ctx: GcMachine) -> Result<bool> {
        let records = match self.records.as_ref() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_ref().unwrap()
            }
        };
        Ok(!records.reader().is_done())
    }

    pub fn read(&mut self, ctx: GcMachine) -> Result<Option<GcArray>> {
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
                    } else {
                        row.add_str(field, ctx);
                    }
                }
                Ok(Some(row))
            }
            None => Ok(None),
        }
    }

    #[finalize]
    fn finalize(&mut self, _: GcMachine) {
        self.records.take();
    }
}
