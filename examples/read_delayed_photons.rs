extern crate endf;

use endf::{DelayedPhotonData, ReadError};
use ::std::fs::{File};
use ::std::io::{BufReader};
use ::std::time::{Instant};

fn test() -> Result<(), ReadError> {
    let start = Instant::now();
    let file = File::open("../n_9437_94-Pu-239.dat")?;
    let mut reader = BufReader::new(file);
    let _delayed_photons = DelayedPhotonData::read_from(&mut reader)?;
    println!("reading took: {:?}", start.elapsed());
    Ok(())
}
fn main() {
    test().expect("failed");
}
