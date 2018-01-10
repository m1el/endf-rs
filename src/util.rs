//! utilities

use super::error::*;

use ::std::io::{BufRead};
use ::std::num::{ParseFloatError};

///
/// Parse ENDF real format into an `f64`.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use endf::{parse_real};
/// assert_eq!(Ok(9.4239e+4), parse_real(" 9.423900+4"))
/// ```
///
pub fn parse_real(s: &str) -> Result<f64, ParseFloatError> {
    let mut buf = String::new();
    parse_real_buf(s, &mut buf)
}

fn parse_real_buf(s: &str, buf: &mut String) -> Result<f64, ParseFloatError> {
    if s.find('e').is_some() {
        return s.trim().parse();
    }

    buf.truncate(0);

    let mut s = s.trim();
    match s.chars().next() {
        Some(c @ '-') | Some(c @ '+') => {
            buf.push(c);
            s = &s[1..];
        },
        _ => {},
    }

    let pos = s.find(|c| c == '+' || c == '-');
    match pos {
        None => {
            buf.push_str(s);
        },
        Some(x) => {
            buf.push_str(&s[..x]);
            buf.push('e');
            buf.push_str(&s[x..]);
        }
    }
    buf.parse()
}

///
/// Read list of reals into a provided buffer.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use endf::{parse_real_row, ReadError};
///
/// const REALS_EXAMPLE: &str = concat!(
///     " 6.15077-10 1.41078-10 1.323138-8",
///     " 1.205944-8 1.093930-8 9.896124-9",
///     "943735 18 6342");
///
/// let mut buf: Vec<f64> = Vec::new();
///
/// let num = parse_real_row(REALS_EXAMPLE, &mut buf)
///         .expect("could not parse reals list");
///
/// assert_eq!(6, num);
///
/// let expect = vec![
///     6.15077e-10, 1.41078e-10, 1.323138e-8,
///     1.205944e-8, 1.093930e-8, 9.896124e-9];
///
/// assert_eq!(expect, buf);
/// ```
///
pub fn parse_real_row(s: &str, buf: &mut Vec<f64>)
    -> Result<usize, ReadError>
{
    let mut tmp = String::new();
    parse_real_row_buf(s, buf, &mut tmp)
}

/// Parse real list with using a scratch buffer
pub fn parse_real_row_buf(mut s: &str, buf: &mut Vec<f64>, tmp: &mut String)
    -> Result<usize, ReadError>
{
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }

    let mut n = 0;
    for _ in 0..6 {
        let w = s[..11].trim();
        s = &s[11..];
        if w.is_empty() {
            return Ok(n);
        }
        n += 1;
        buf.push(parse_real_buf(w, tmp)?);
    }
    Ok(n)
}

/// Read a list of N entries from a file
pub fn read_real_list<F>(source: &mut F, n: usize)
    -> Result<Vec<f64>, ReadError>
    where F: BufRead
{
    let mut buf = String::new();
    let row_count =
        if n == 0 { 0 }
        else { 1 + (n - 1) / 6 };

    let mut rv: Vec<f64> = Vec::new();
    let mut tmp = String::new();
    for _ in 0..row_count {
        buf.truncate(0);
        source.read_line(&mut buf)?;
        parse_real_row_buf(&buf, &mut rv, &mut tmp)?;
    }
    Ok(rv)
}

///
/// Read list of reals ints a provided buffer.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// use endf::{parse_int_list, ReadError};
///
/// const INTS_EXAMPLE: &str = concat!(
///     "          1          2          3",
///     "                                 ",
///     "943735 18 6342");
///
/// let mut buf: Vec<i32> = Vec::new();
///
/// let num = parse_int_list(INTS_EXAMPLE, &mut buf)
///         .expect("could not parse reals list");
///
/// assert_eq!(3, num);
///
/// let expect = vec![1, 2, 3];
///
/// assert_eq!(expect, buf);
/// ```
///
pub fn parse_int_list(mut s: &str, buf: &mut Vec<i32>)
    -> Result<usize, ReadError>
{
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }
    let mut n = 0;
    for _ in 0..6 {
        let w = s[..11].trim();
        s = &s[11..];
        if w.is_empty() {
            return Ok(n);
        }
        n += 1;
        buf.push(w.parse()?);
    }
    Ok(n)
}

///
/// Parse record identifier tuple `(material, file, section, index)`
///
/// # Examples
/// Basic usage:
///
/// ```rust
/// use endf::{parse_record_ident};
///
/// const REALS_EXAMPLE: &str = concat!(
///     " 6.15077-10 1.41078-10 1.323138-8",
///     " 1.205944-8 1.093930-8 9.896124-9",
///     "943735 18 6342");
///
/// let ident = parse_record_ident(REALS_EXAMPLE)
///         .expect("could not parse record identifier");
///
/// assert_eq!((9437, 35, 18, 6342), ident);
/// ```
///
pub fn parse_record_ident(s: &str)
    -> Result<(i32, i32, i32, i32), ReadError>
{
    if s.len() < 80 {
        return Err(ReadError::RecordTooShort);
    }
    let s = &s[66..];
    let mat: i32 = s[0..4].trim().parse()?;
    let mf: i32 = s[4..6].trim().parse()?;
    let mt: i32 = s[6..9].trim().parse()?;
    let ns: i32 = s[9..].trim().parse()?;
    Ok((mat, mf, mt, ns))
}

/// Parse CONT record (section 0.6.3.2)
///
/// Basic usage:
///
/// ```rust
/// use endf::{parse_cont_record};
/// const CONT_RECORD: &str = concat!(
///     " 9.423900+4 2.369986+2          1",
///     "          1          0          5",
///     "9437 1451    1");
/// let cont = parse_cont_record(CONT_RECORD)
///         .expect("could not parse cont record");
/// assert_eq!((9.423900e+4, 2.369986e+2, 1, 1, 0, 5), cont);
/// ```
pub fn parse_cont_record(mut s: &str)
    -> Result<(f64, f64, i32, i32, i32, i32), ReadError>
{
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }
    let c1: f64 = parse_real(&s[..11])?; s = &s[11..];
    let c2: f64 = parse_real(&s[..11])?; s = &s[11..];
    let l1: i32 = s[..11].trim().parse()?; s = &s[11..];
    let l2: i32 = s[..11].trim().parse()?; s = &s[11..];
    let n1: i32 = s[..11].trim().parse()?; s = &s[11..];
    let n2: i32 = s[..11].trim().parse()?;
    Ok((c1, c2, l1, l2, n1, n2))
}

/// Parse TEXT record (section 0.6.3.1)
///
/// Basic usage:
///
/// ```rust
/// use endf::{parse_text_record};
/// const TEXT_RECORD: &str = concat!(
///     "   Modifications were made to MT=458 based",
///     " on a new analysis by   9437 1451   91");
///
/// let text = parse_text_record(TEXT_RECORD)
///         .expect("could not parse text record");
/// assert_eq!(&text, concat!(
///     "   Modifications were made to MT=458",
///     " based on a new analysis by   "))
/// ```
pub fn parse_text_record(s: &str) -> Result<String, ReadError> {
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }
    Ok(s[..66].to_owned())
}

///
/// Seek to the specified `(material, file, section)` tuple.
///
/// Returns `ReadError::Eof` if we've reached the end.
///
pub fn seek_to_tuple_mat(
    source: &mut BufRead,
    material: i32, file: i32, section: i32,
) -> Result<String, ReadError>
{
    let mut buf = String::new();
    loop {
        buf.truncate(0);
        if source.read_line(&mut buf)? == 0 {
            return Err(ReadError::Eof);
        }
        let (cur_mat, cur_mf, cur_mt, _) = parse_record_ident(&buf)?;
        if (cur_mat, cur_mf, cur_mt) == (material, file, section) {
            return Ok(buf);
        }
    }
}

///
/// Seek to the specified `(file, section)` tuple.
///
/// Returns `ReadError::Eof` if we've reached the end.
///
pub fn seek_to_tuple(source: &mut BufRead, file: i32, section: i32)
    -> Result<String, ReadError>
{
    let mut buf = String::new();
    loop {
        buf.truncate(0);
        if source.read_line(&mut buf)? == 0 {
            return Err(ReadError::Eof);
        }
        let (_, cur_mf, cur_mt, _) = parse_record_ident(&buf)?;
        if (cur_mf, cur_mt) == (file, section) {
            return Ok(buf);
        }
    }
}
