use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str};
use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::path::PathBuf;
use indicatif::ProgressBar;

const INVALID_CHAR: &str = "\u{feff}";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PackageReference {
  #[serde(alias = "Include", default)]
  pub include: String,

  #[serde(alias = "Version", default)]
  pub version: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ItemGroup {
  #[serde(alias = "PackageReference", default)]
  pub dependencies: Vec<PackageReference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project {
  #[serde(rename = "ItemGroup", default)]
  pub item_groups: Vec<ItemGroup>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Deps {
  pub name: String,
  pub dependencies: Vec<PackageReference>,
}

impl Deps {
  fn new(in_name: &str) -> Deps {
      Deps {
          name : in_name.to_string(), //in_name.to_os_string().into_string().unwrap(),
          dependencies : vec!(),
      }
  }

  fn from_proj_str(name: &str, proj: &mut Project) -> Result<Deps, serde_xml_rs::Error> {
    let mut deps = Deps::new(name); 

    for item_group in &mut proj.item_groups {
      if item_group.dependencies.len() > 0 {
        deps.dependencies.append(&mut item_group.dependencies);
      }
    }

    Ok(deps)
  }

  pub fn vec_from_filepaths(paths: Vec<PathBuf>) -> Result<Vec<Deps>, std::io::Error> {
    let mut deps_vec: Vec<Deps> = vec!();
    for path in paths{
      let source = read_csproj(&path)?;
      match from_str(&source) {
        Err(why) => println!("{:?}", why),
        Ok(mut proj) => {
          let deps = 
            Deps::from_proj_str(
              &path.file_stem().unwrap().to_os_string().into_string().unwrap(), 
              &mut proj).unwrap();
              deps_vec.push(deps);
        }
      }
    }
    Ok(deps_vec)
  }
}

#[derive(Debug, Serialize)]
pub struct ProjectCollection<'a> {
  pub content: &'a Vec<Deps>,
  pub project_count: &'a usize,
}

pub fn find_projects<'a>(input_path: PathBuf, paths: &'a mut Vec<PathBuf>, bar: &mut ProgressBar, is_rec_search: bool) {
  bar.inc(1);
  if input_path.is_file() {
    if is_project(&input_path) {
      paths.push(input_path);
    }
  }
  else {
    let entries = read_dir(input_path).unwrap();
    for dir in entries {
      match dir {
        Err(why) => println!("{:?}", why),
        Ok(entry) => {
          if is_rec_search && entry.path().is_dir() {
            bar.set_message(&format!("Searching /{}", entry.path().file_stem().unwrap().to_os_string().into_string().unwrap()));
            find_projects(entry.path(), paths, bar, is_rec_search);
          }
          if is_project(&entry.path()){
            bar.set_message(&format!("Found {}", entry.path().file_stem().unwrap().to_os_string().into_string().unwrap()));
            paths.push(entry.path());
          }
        }
      }
    }
  }
}

fn read_csproj(path: &PathBuf) -> Result<String, std::io::Error> {
  let mut file = File::open(path)?;
  let mut proj = String::new();
  file.read_to_string(&mut proj)?;
  Ok(proj.trim_start_matches(INVALID_CHAR).to_string())
}

fn is_project(entry: &PathBuf) -> bool {
  if let Some(extension) = entry.extension(){
    return extension == "csproj";
  }
  false
}
