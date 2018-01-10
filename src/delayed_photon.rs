//! Delayed Photon Data (`MF=1, MT=460`)
//!
//! This section is described in Chapter 1.6 of ENDF-6 Formats Manual

use ::std::io::{Seek, BufRead};

use error::{ReadError};
use tabular::{Tab1};
use util::{seek_to_tuple, parse_cont_record, read_real_list};

/// Delayed photon data info (Section 1.6.1)
#[derive(Debug)]
pub enum DelayedPhotonData {
    /// Discrete representation (`LO=1`) Section 1.6.1.1
    Discrete(Vec<Tab1>),
    /// Continuous representation (`LO=2`) Section 1.6.1.2
    Continuous(Vec<f64>),
}

impl DelayedPhotonData {
    /// Read delayed photon data from a source
    ///
    /// Example:
    ///
    /// ```rust
    /// use endf::{DelayedPhotonData, ReadError};
    /// use std::fs::{File};
    /// use std::io::{BufReader};
    ///
    /// # fn foo() -> Result<(), ReadError> {
    /// let file = File::open("input_file.dat")?;
    /// let mut reader = BufReader::new(file);
    /// let delayed_photons = DelayedPhotonData::read_from(&mut reader)?;
    /// # Ok(()) }
    /// ```
    pub fn read_from<F>(source: &mut F)
        -> Result<DelayedPhotonData, ReadError>
        where F: Seek+BufRead
    {
        let mut line = seek_to_tuple(source, 1, 460)?;
        let (_, _, lo, _, ng, _) = parse_cont_record(&line)?;
        match lo {
        1 => {
            let mut tabs: Vec<Tab1> = Vec::new();
            for _ in 0..ng {
                tabs.push(Tab1::read_from(source)?);
            }
            Ok(DelayedPhotonData::Discrete(tabs))
        },
        2 => {
            line.truncate(0);
            source.read_line(&mut line)?;
            let (_, _, _, _, nnf, _) = parse_cont_record(&line)?;
            let list = read_real_list(source, nnf as usize)?;
            Ok(DelayedPhotonData::Continuous(list))
        },
        _ => {
            Err(ReadError::Eof)
        }
        }
    }
}
