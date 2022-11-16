use std::{fs::File, path::PathBuf};

use anyhow::Result;
use arrow::compute::kernels::concat::concat_batches;
use parquet::arrow::arrow_reader::ParquetRecordBatchReader;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "broken-arrow",
    about = "demonstrate arrow concat_batches SEGFAULT"
)]
struct Opt {
    #[structopt()]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let mut batches = Vec::new();
    let opt = Opt::from_args();
    for path in opt.files.iter() {
        println!("Reading {path:?}");
        let file = File::open(path)?;
        let reader = ParquetRecordBatchReader::try_new(file, 1024)?;
        for batch in reader {
            batches.push(batch?);
        }
    }

    let schema = batches[0].schema();
    let _merged = concat_batches(&schema, &batches)?;

    Ok(())
}
