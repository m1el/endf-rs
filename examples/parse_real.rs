extern crate endf;
use endf::{parse_real, parse_real_row, parse_record_ident};
const RECORD_EXAMPLE: &str = " 9.423900+4 2.369986+2          1          1          0          59437 1451    1";
const REALS_EXAMPLE: &str  = " 6.15077-10 1.41078-10 1.323138-8 1.205944-8 1.093930-8 9.896124-9943735 18 6342";
const FLOAT_EXAMPLE: &str = " 9.423900+4";
fn parse_example() -> std::result::Result<(), endf::ReadError> {
    println!("float source: `{}`", FLOAT_EXAMPLE);
    println!("parsing a float: {:?}", parse_real(FLOAT_EXAMPLE)?);
    println!("record source: `{}`", RECORD_EXAMPLE);
    println!("parsing record id: {:?}", parse_record_ident(RECORD_EXAMPLE)?);
    let mut v = Vec::new();
    parse_real_row(REALS_EXAMPLE, &mut v)?;
    println!("record source: `{}`", REALS_EXAMPLE);
    println!("parsing list: {:?}", v);
    Ok(())
}
fn main() {
    parse_example().expect("parsing failed!");
}
