# endf-rs

Parsing ENDF-6 files in Rust

This was born of an attempt to read [ENDF files](https://www.bnl.gov/isd/documents/70393.pdf).

Reading for Description and delayed photon data is implemented, everything else should be easily readable with helper functions already implemented.

Performance: it takes ~250ms to read ~240K pairs of points in tabulated data on i7-3517U@1.90GHz.

# Documentation

See examples and use `cargo doc` to build documentation for this crate.
