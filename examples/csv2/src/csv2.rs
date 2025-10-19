use std::fs::File;

use greycat::prelude::*;

#[greycat_type]
pub struct CsvReader {
    records: Option<Box<csv::StringRecordsIntoIter<File>>>,
}

impl CsvReader {
    pub fn finalize(&mut self, _: GcMachine) {
        self.records.take();
    }

    pub fn can_read(&mut self, ctx: GcMachine) -> GcResult<bool> {
        let records = match self.records.as_ref() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_ref().unwrap()
            }
        };
        Ok(!records.reader().is_done())
    }

    pub fn read(&mut self, ctx: GcMachine) -> GcResult<Option<impl AsGcValue>> {
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
            None => Ok(Option::<GcArray>::None),
        }
    }

    fn initialize_reader(&mut self, ctx: GcMachine) -> GcResult<()> {
        let file = File::open(self.path(ctx).as_str())?;
        let reader = csv::ReaderBuilder::new()
            // TODO map builder options to CsvFormat
            .has_headers(true)
            .flexible(true)
            .buffer_capacity(8192)
            .from_reader(file);
        let records = reader.into_records();
        self.records.replace(Box::new(records));
        Ok(())
    }
}
