//! Utilities for reading tabular data.

extern crate ndarray;

//use ::std::convert::{TryFrom};
use ::std::io::{BufRead};
use self::ndarray::{Array, Array2};

use error::{ReadError};
use util::{parse_cont_record, parse_int_list, parse_real_row_buf};

/// Interpolation Scheme numbers
/// described in Chapter 0.5.2.1 and Table 16
#[derive(Debug)]
pub enum InterpolationScheme {
    /// y is constant in x
    ConstantHistogram,
    /// y is linear in x
    LinearLinear,
    /// y is linear in ln(x)
    LinearLog,
    /// ln(y) is linear in x
    LogLinear,
    /// ln(y) is linear in ln(x)
    LogLog,
    /// special one-dimensional interpolation law,
    /// used for charged-particle cross sections only
    Special
    // TODO: implement 11..15, 21..25
}

impl InterpolationScheme {
    /// Parse interpolation scheme from int
    pub fn try_from(x: i32) -> Result<InterpolationScheme, ReadError> {
        use InterpolationScheme::*;
        let rv = match x {
            1 => ConstantHistogram,
            2 => LinearLinear,
            3 => LinearLog,
            4 => LogLinear,
            5 => LogLog,
            6 => Special,
            _ => return Err(ReadError::InvalidInterpolation),
        };
        Ok(rv)
    }
}

impl Into<i32> for InterpolationScheme {
    fn into(self) -> i32 {
        use InterpolationScheme::*;
        match self {
            ConstantHistogram => 1,
            LinearLinear => 2,
            LinearLog => 3,
            LogLinear => 4,
            LogLog => 5,
            Special => 6,
        }
    }
}

/// Interpolation interval definition
#[derive(Debug)]
pub struct InterpolationInterval {
    /// Scheme used for given interval
    pub scheme: InterpolationScheme,
    /// Lower bound, inclusive
    pub start: usize,
    /// Upper bound, exclusive
    pub end: usize,
}

/// TAB1 Record - interpolated tabular data
///
/// As defined in Section 0.6.3.7
#[derive(Debug)]
pub struct Tab1 {
    /// Additional data, which is usually dismissed (C1, C2, L1, L2)
    pub head: (f64, f64, i32, i32),
    /// List of interpolation intervals
    pub intervals: Vec<InterpolationInterval>,
    /// Raw tabulated data for interpolation
    pub data: Array2<f64>,
}

impl Tab1 {
    /// Read tabulated data from source
    ///
    /// Example:
    ///
    /// ```rust
    /// use endf::{Tab1};
    /// use std::fs::{File};
    /// use std::io::{BufReader};
    ///
    /// # fn foo() -> Result<(), endf::ReadError> {
    /// let file = File::open("input.dat")?;
    /// let mut reader = BufReader::new(file);
    /// let tab = Tab1::read_from(&mut reader)?;
    /// # Ok(()) }
    pub fn read_from<F>(source: &mut F)
        -> Result<Tab1, ReadError>
        where F: BufRead
    {
        let mut buf = String::new();
        source.read_line(&mut buf)?;
        let (c1, c2, l1, l2, range_count, point_count) = parse_cont_record(&buf)?;
        let head = (c1, c2, l1, l2);
        let range_count = range_count as usize;
        let point_count = point_count as usize;
        // ceil of integer division
        let range_lines = 1 + ((range_count * 2 - 1) / 6);
        let point_lines = 1 + ((point_count * 2 - 1) / 6);

        let mut tmp: Vec<i32> = Vec::new();
        let mut intervals: Vec<InterpolationInterval> = Vec::new();
        for _ in 0..range_lines {
            buf.truncate(0);
            source.read_line(&mut buf)?;
            parse_int_list(&buf, &mut tmp)?;
        }
        if tmp.len() != range_count * 2 {
            return Err(ReadError::InvalidElementCount);
        }

        let mut prev = 0;
        for w in tmp.windows(2) {
            intervals.push(InterpolationInterval {
                scheme: InterpolationScheme::try_from(w[1])?,
                start: prev,
                end: w[0] as usize,
            });
            prev = w[0] as usize;
        }

        let mut raw: Vec<f64> = Vec::new();
        let mut scratch = String::new();
        for _ in 0..point_lines {
            buf.truncate(0);
            source.read_line(&mut buf)?;
            parse_real_row_buf(&buf, &mut raw, &mut scratch)?;
        }
        if raw.len() != point_count * 2 {
            return Err(ReadError::InvalidElementCount);
        }

        let data = Array::from_vec(raw).into_shape((point_count, 2))
                .expect("invalid array reshape?");

        Ok(Tab1 { head, intervals, data })
    }
}

/// TAB2 Record - interpolated 2D tabular data
///
/// As defined in Section 0.6.3.8
#[derive(Debug)]
pub struct Tab2 {
    /// Additional data, which is usually dismissed (C1, C2, L1, L2)
    pub head: (f64, f64, i32, i32),
    /// List of interpolation intervals
    pub intervals: Vec<InterpolationInterval>,
    /// Raw interpolation slices
    pub data: Vec<Tab1>,
}

impl Tab2 {
    /// Read 2D tabulated data from source
    ///
    /// Example:
    ///
    /// ```rust
    /// use endf::{Tab2};
    /// use std::fs::{File};
    /// use std::io::{BufReader};
    ///
    /// # fn foo() -> Result<(), endf::ReadError> {
    /// let file = File::open("input.dat")?;
    /// let mut reader = BufReader::new(file);
    /// let tab = Tab2::read_from(&mut reader)?;
    /// # Ok(()) }
    pub fn read_from<F>(source: &mut F)
        -> Result<Tab2, ReadError>
        where F: BufRead
    {
        let mut buf = String::new();
        source.read_line(&mut buf)?;
        let (c1, c2, l1, l2, range_count, slice_count) = parse_cont_record(&buf)?;
        let head = (c1, c2, l1, l2);
        let range_count = range_count as usize;
        let slice_count = slice_count as usize;
        // ceil of integer division
        let range_lines = 1 + ((range_count - 1) / 6);

        let mut tmp: Vec<i32> = Vec::new();
        let mut intervals: Vec<InterpolationInterval> = Vec::new();
        for _ in 0..range_lines {
            buf.truncate(0);
            source.read_line(&mut buf)?;
            parse_int_list(&buf, &mut tmp)?;
        }
        if tmp.len() != range_count * 2 {
            return Err(ReadError::InvalidElementCount);
        }

        let mut prev = 0;
        for w in tmp.windows(2) {
            intervals.push(InterpolationInterval {
                scheme: InterpolationScheme::try_from(w[1])?,
                start: prev,
                end: w[0] as usize,
            });
            prev = w[0] as usize;
        }

        let mut data: Vec<Tab1> = Vec::new();
        for _ in 0..slice_count {
            data.push(Tab1::read_from(source)?);
        }

        Ok(Tab2 { head, intervals, data })
    }
}
