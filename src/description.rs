//! Descriptive Data and Directory (`MF=1, MT=451`)

use ::std::io::{BufRead, Seek};
use error::{ReadError};
use util::{
    parse_text_record, parse_record_ident,
    parse_cont_record, seek_to_tuple};

/// Descriptive data section struct `MF=1, MT=451`
///
/// This section is described in Chapter 1.1 of ENDF-6 Formats Manual
#[derive(Debug)]
pub struct DescriptionCard {
    /// `(Z, A)` Designation of the original nuclide. `ZA = 1000 * Z + A`
    pub ZA: f64,
    /// AWR: Ratio of mass of atom (or molecule) to that of the neutron.
    pub AWR: f64,
    /// LRP: Indicates whether resolved and/or unresolved
    /// resonance parameters given in File 2.
    pub LRP: i32,
    /// LFI: Indicates whether this material is fissionable.
    pub LFI: i32,
    /// NLIB: Library identifier.
    pub NLIB: i32,
    /// NMOD: Modification number.
    pub NMOD: i32,

    /// ELIS: Excitation energy of the target nucleus.
    pub ELIS: f64,
    /// STA: Target stability flag.
    pub STA: f64,
    /// LIS: State number of the target nucleus
    /// (for materials that represent nuclides).
    pub LIS: i32,
    ///  LISO: Isomeric state number of the target nucleus.
    pub LISO: i32,
    /// NFOR: Library format.
    pub NFOR: i32,

    /// AWI: Projectile mass in neutron units.
    pub AWI: f64,
    /// EMAX: Upper limit of energy range for evaluation.
    pub EMAX: f64,
    /// LREL: Release number
    pub LREL: i32,
    /// NSUB: Sub-library number
    pub NSUB: i32,
    /// NVER: Library version number.
    pub NVER: i32,

    /// TEMP: Target temperature.
    pub TEMP: f64,
    /// LDRV: Distinguishes between different evaluations
    /// with the same material keys
    pub LDRV: i32,
    /// NWD: Number of elements in the text section.
    pub NWD: i32,
    /// NXC: Number of the sections to be found in the dictionary
    pub NXC: i32,

    /// ZSYMAM: Text representation of material Z-chemical symbol-Astate.
    pub ZSYMAM: String,
    /// ALAB: Mnemonic of laboratory originating evaluation.
    pub ALAB: String,
    /// EDATE: Date of evaluation.
    pub EDATE: String,
    /// AUTH: Author of evaluation.
    pub AUTH: String,

    /// REF: Reference to evaluation.
    pub REF: String,
    /// DDATE: Original distribution date of the evaluation.
    pub DDATE: String,
    /// RDATE: Date and number of last revision.
    pub RDATE: String,
    /// ENDATE: Master file entry date (yyyymmdd).
    pub ENDATE: i32,

    /// description: textual comments
    pub description: String,
    /// directory: list of sections in this file
    pub directory: Vec<DirectoryEntry>,
}

impl DescriptionCard {
    /// Read Descriptive Data and Directory from a given file
    ///
    /// Example:
    ///
    /// ```rust
    /// use endf::{DescriptionCard, ReadError};
    /// use std::fs::{File};
    /// use std::io::{BufReader};
    ///
    /// # fn foo() -> Result<(), ReadError> {
    /// let file = File::open("input_file.dat")?;
    /// let mut reader = BufReader::new(file);
    /// let description = DescriptionCard::read_from(&mut reader)?;
    /// # Ok(()) }
    /// ```
    pub fn read_from<F>(source: &mut F)
        -> Result<DescriptionCard, ReadError>
        where F: Seek+BufRead
    {
        use std::io::{SeekFrom};
        source.seek(SeekFrom::Start(0))?;

        let mut line = seek_to_tuple(source, 1, 451)?;
        let (ZA, AWR, LRP, LFI, NLIB, NMOD) = parse_cont_record(&line)?;
        line.truncate(0);
        source.read_line(&mut line)?;
        let (ELIS, STA, LIS, LISO, _, NFOR) = parse_cont_record(&line)?;
        line.truncate(0);
        source.read_line(&mut line)?;
        let (AWI, EMAX, LREL, _, NSUB, NVER) = parse_cont_record(&line)?;
        line.truncate(0);
        source.read_line(&mut line)?;
        let (TEMP, _, LDRV, _, NWD, NXC) = parse_cont_record(&line)?;
        line.truncate(0);
        source.read_line(&mut line)?;
        let (ZSYMAM, ALAB, EDATE, AUTH) = parse_zsym_row(&line)?;
        line.truncate(0);
        source.read_line(&mut line)?;
        let (REF, DDATE, RDATE, ENDATE) = parse_ref_row(&line)?;
        let mut description = String::new();

        for _ in 0..(NWD - 2) {
            line.truncate(0);
            source.read_line(&mut line)?;
            description.push_str("\n");
            description.push_str(&parse_text_record(&line)?);
        }

        let mut directory = Vec::new();
        for _ in 0..NXC {
            line.truncate(0);
            source.read_line(&mut line)?;
            directory.push(parse_directory_entry(&line)?);
        }
        line.truncate(0);
        source.read_line(&mut line)?;
        let (_, _, section, index) = parse_record_ident(&line)?;
        if (section, index) != (0, 99_999) {
            return Err(ReadError::MissingSectionTerminator);
        }

        Ok(DescriptionCard {
            ZA, AWR, LRP, LFI, NLIB, NMOD,
            ELIS, STA, LIS, LISO, NFOR,
            AWI, EMAX, LREL, NSUB, NVER,
            TEMP, LDRV, NWD, NXC,
            ZSYMAM, ALAB, EDATE, AUTH,
            REF, DDATE, RDATE, ENDATE,
            description,
            directory,
        })
    }

    /// Split ZA into charge and baryon count
    pub fn get_za(&self) -> (i32, i32) {
        let za = self.ZA as i32;
        (za / 1000, za % 1000)
    }
}

/// Section directory descriptor
#[derive(Debug)]
pub struct DirectoryEntry {
    /// MF: File number.
    pub MF: i32,
    /// MT: Reaction type number, or, covariance file section identifier.
    pub MT: i32,
    /// NC: Number of records in the section.
    pub NC: i32,
    /// MOD: Modification indicator.
    pub MOD: i32,
}

fn parse_zsym_row(mut s: &str)
    -> Result<(String, String, String, String), ReadError>
{
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }
    let ZSYMAM = s[..11].to_owned();
    s = &s[11..];
    let ALAB = s[..11].to_owned();
    s = &s[11..];
    let EDATE = s[..11].to_owned();
    s = &s[11..];
    let AUTH = s[..33].to_owned();
    Ok((ZSYMAM, ALAB, EDATE, AUTH))
}

fn parse_ref_row(mut s: &str)
    -> Result<(String, String, String, i32), ReadError>
{
    if s.len() < 66 {
        return Err(ReadError::RecordTooShort);
    }
    let REF = s[..22].to_owned();
    s = &s[22..];
    let DDATE = s[..11].to_owned();
    s = &s[11..];
    let RDATE = s[..11].to_owned();
    s = &s[11..];
    let ENDATE: i32 = s[11..22].trim().parse()?;
    Ok((REF, DDATE, RDATE, ENDATE))
}

fn parse_directory_entry(mut s: &str)
    -> Result<DirectoryEntry, ReadError>
{
    s = &s[22..];
    let MF: i32 = s[..11].trim().parse()?;
    s = &s[11..];
    let MT: i32 = s[..11].trim().parse()?;
    s = &s[11..];
    let NC: i32 = s[..11].trim().parse()?;
    s = &s[11..];
    let MOD: i32 = s[..11].trim().parse()?;
    Ok(DirectoryEntry {
        MF, MT, NC, MOD
    })
}
