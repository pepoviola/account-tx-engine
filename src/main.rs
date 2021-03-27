use std::env;
use std::error::Error;
use std::fs::File;

mod account;
mod engine;
mod transaction;
mod transaction_row;

/// Helper function to parse and validate the required args
/// We expect one argument and this should be an .csv file.
fn get_file_name_to_process() -> Result<String, Box<dyn Error>> {
    if let Some(file_name) = env::args().nth(1) {
        match file_name.ends_with(".csv") {
            true => Ok(file_name),
            false => Err(From::from(format!(
                "Invalid filename {}, expected an .csv file",
                file_name
            ))),
        }
    } else {
        Err(From::from("Expected 1 argument, but got none"))
    }
}

fn get_reader_from_file(file_name: &str) -> Result<csv::Reader<File>, Box<dyn Error>> {
    let reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(file_name)?;

    Ok(reader)
}

#[derive(Debug)]
pub struct Amount {
    inner: u64,
}

impl Amount {
    pub fn new(inner: u64) -> Amount {
        Amount { inner }
    }

    pub fn value(&self) -> u64 {
        self.inner
    }

    pub fn from_input(amount: f64) -> Amount {
        let inner = (amount * 1.0e+4_f64) as u64;
        Amount { inner }
    }

    pub fn to_output(&self) -> f64 {
        (self.inner as f64 / 1.0e+4_f64) as f64
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = get_file_name_to_process()?;
    let reader = get_reader_from_file(&file_name)?;
    let mut engine = engine::PaymentEngine::new();
    engine.process_from_reader(reader)?;
    engine.output_report();
    Ok(())
}
