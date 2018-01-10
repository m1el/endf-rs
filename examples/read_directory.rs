extern crate endf;

use endf::{DescriptionCard, ReadError};
use ::std::fs::{File};
use ::std::io::{BufReader};

fn test() -> Result<(), ReadError> {
    let file = File::open("../n_9437_94-Pu-239.dat")?;
    let mut reader = BufReader::new(file);
    let description = DescriptionCard::read_from(&mut reader)?;
    println!("{:?}", description);
    Ok(())
}
fn main() {
    test().expect("failed");
}
