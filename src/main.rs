#![allow(dead_code, unused_variables)]

use structopt::StructOpt;
use std::path::PathBuf;
use csdeps::{rec_read_dir};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Directory to search for project files
    #[structopt(name = "DIR", parse(from_os_str))]
    dir: PathBuf,

    /// Output using json format
    #[structopt(short, long)]
    json: bool,
}


fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    rec_read_dir(opt.dir.as_path(), opt.json);

    Ok(())
}
