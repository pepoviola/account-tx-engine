use std::env;
use std::error::Error;
use std::fs::File;

mod account;
mod engine;
mod transaction;
mod transaction_row;
mod amount;

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

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = get_file_name_to_process()?;
    let reader = get_reader_from_file(&file_name)?;
    let mut engine = engine::PaymentEngine::new();
    engine.process_from_reader(reader)?;
    engine.output_report();
    Ok(())
}
