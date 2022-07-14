use oxipng::optimize;
use oxipng::{InFile, IndexSet, OutFile, PngError};
use std::path::Path;
use std::path::PathBuf;

extern crate oxipng;

pub type PngResult<T> = Result<T, PngError>;

pub struct Compressor {}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor {}
    }

    pub fn compress(&self, input: &str) -> PngResult<()> {
        let input = PathBuf::from(input);
        let (output, opts) = get_opts(&input);
        let result = optimize(&InFile::Path(input), &output, &opts);

        match result {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
}

fn get_opts(input: &Path) -> (OutFile, oxipng::Options) {
    let mut options = oxipng::Options {
        force: true,
        ..Default::default()
    };
    let mut filter = IndexSet::new();
    filter.insert(0);
    options.filter = filter;

    (
        OutFile::Path(Some(input.with_extension("out.png"))),
        options,
    )
}

#[test]
fn test_compress() {
    let engine = Compressor::new();
    let result = engine.compress("compress.png");

    match result {
        Ok(_data) => (println!("Success!")),
        Err(e) => panic!("Error {}", e),
    }
}
