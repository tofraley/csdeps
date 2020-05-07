#![allow(dead_code, unused_variables)]

use structopt::StructOpt;
use std::path::PathBuf;
use csdeps::{Deps, ProjectCollection, rec_read_dir};

#[derive(StructOpt, Debug)]
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
    let deps = rec_read_dir(opt.dir.as_path())?;
    handle_deps_with_opt(deps, opt);
    Ok(())
}

fn handle_deps_with_opt(deps: Vec<Deps>, opt: Opt) {
    if opt.json  {
        let coll: ProjectCollection = ProjectCollection {
          content: &deps,
          project_count: &deps.len(),
        };
        
        let j = serde_json::to_string_pretty(&coll).unwrap();
        println!("{}", j);
    }
    else {
        for dep in deps.iter() {
          println!("\nProject name: {}", dep.name);
          for reference in dep.dependencies.iter() {
            println!("{} version: {}", reference.include, reference.version);
          }
        }
    }
}