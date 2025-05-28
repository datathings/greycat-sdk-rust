use std::fs::File;

use greycat2::{self as gc, greycat, AsValue};

#[greycat]
#[repr(C)]
pub struct CsvReader {
    records: Option<csv::StringRecordsIntoIter<File>>,
}

impl CsvReader {
    pub fn __initialize(&mut self, _ctx: gc::Machine) {
        std::mem::forget(self.records.take());
        self.__initialized = true
    }

    pub fn __finalize(&mut self) {
        let _ = self.records.take();
    }

    pub fn can_read(&mut self, ctx: gc::Machine) -> Result<bool, String> {
        let records = match self.records.as_ref() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_ref().unwrap()
            }
        };
        Ok(!records.reader().is_done())
    }

    pub fn read(&mut self, ctx: gc::Machine) -> Result<Option<impl AsValue>, String> {
        let records = match self.records.as_mut() {
            Some(records) => records,
            None => {
                self.initialize_reader(ctx)?;
                self.records.as_mut().unwrap()
            }
        };

        match records.next() {
            Some(record) => {
                let record = record.map_err(|err| err.to_string())?;
                let mut row = gc::Array::new(ctx);
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
            None => Ok(Option::<gc::Array>::None),
        }
    }

    fn initialize_reader(&mut self, ctx: gc::Machine) -> Result<(), String> {
        let file = File::open(self.path(ctx).as_str()).map_err(|err| err.to_string())?;
        let reader = csv::ReaderBuilder::new()
            // TODO map builder options to CsvFormat
            .has_headers(true)
            .flexible(true)
            .buffer_capacity(8192)
            .from_reader(file);
        let records = reader.into_records();
        self.records.replace(records);
        Ok(())
    }
}
