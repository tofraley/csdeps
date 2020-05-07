#![allow(dead_code, unused_variables)]

use structopt::StructOpt;
use std::path::PathBuf;
use csdeps::{Deps, rec_read_dir};
use serde::{Serialize};

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

#[derive(Debug, Serialize)]
struct ProjectCollection<'a> {
    pub content: &'a Vec<Deps>,
    pub project_count: &'a usize,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let deps: Vec<Deps> = rec_read_dir(opt.dir.as_path());

    handle_deps_with_opts(deps, opt);

    Ok(())
}

fn handle_deps_with_opts(deps: Vec<Deps>, opt: Opt) {
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