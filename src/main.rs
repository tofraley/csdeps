#![allow(dead_code, unused_variables)]

use structopt::StructOpt;
use std::path::PathBuf;
use csdeps::{Deps, ProjectCollection, find_projects};
use indicatif::ProgressBar;

#[derive(StructOpt, Debug)]
struct Opt {
  /// Directory to search for project files
  #[structopt(name = "DIR", parse(from_os_str))]
  dir: PathBuf,

  /// Output using json format
  #[structopt(short="j", long)]
  json: bool,

  /// Search child directories recursively
  #[structopt(short="r", long)]
  recursive: bool,
}

fn main() -> std::io::Result<()> {
  let opt = Opt::from_args();
  let mut paths: Vec<PathBuf> = vec!();
  let mut bar = ProgressBar::new_spinner();
  find_projects(opt.dir, &mut paths, &mut bar, opt.recursive);
  bar.finish_and_clear();
  let deps = Deps::vec_from_filepaths(paths)?;
  handle_deps_with_opt(deps, opt.json);
  Ok(())
}

fn handle_deps_with_opt(deps: Vec<Deps>, use_json: bool) {
  if use_json  {
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