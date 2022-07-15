use oxipng::{optimize, optimize_from_memory};
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

    pub fn compress_from_memory(&self, input: &[u8]) -> PngResult<Vec<u8>> {
        let (opts) = self.get_options();
        let result = optimize_from_memory(&input, &opts);

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    fn get_options(&self) -> oxipng::Options {
        let mut options = oxipng::Options {
            force: true,
            ..Default::default()
        };

        let mut filter = IndexSet::new();
        filter.insert(0);
        options.filter = filter;
        return options;
    }

    fn get_opts(&self, input: &Path) -> (PathBuf, oxipng::Options) {
        let mut output = PathBuf::from(input);
        output.set_extension("out.png");
        let opts = self.get_options();
        (output, opts)
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
