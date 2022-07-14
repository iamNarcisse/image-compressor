use oxipng::optimize;
use oxipng::{InFile, IndexSet, OutFile, PngError};

extern crate oxipng;

pub type PngResult<T> = Result<T, PngError>;

pub struct Compressor {}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor {}
    }

    pub fn compress(&self, input: &InFile) -> PngResult<()> {
        let (output, opts) = get_opts(input);
        let result = optimize(&input, &output, &opts);
        match result {
            Ok(data) => Ok(()),
            Err(e) => panic!("{}", e),
        }
    }
}

fn get_opts(input: &InFile) -> (OutFile, oxipng::Options) {
    let mut options = oxipng::Options {
        force: true,
        ..Default::default()
    };
    let mut filter = IndexSet::new();
    filter.insert(0);
    options.filter = filter;

    (OutFile::Path(None), options)
}
