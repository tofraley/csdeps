use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str};
use std::fs::{File, DirEntry, read_dir};
use std::io::prelude::*;
use std::path::Path;

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
}

#[derive(Debug, Serialize)]
pub struct ProjectCollection<'a> {
  pub content: &'a Vec<Deps>,
  pub project_count: &'a usize,
}

pub fn rec_read_dir(input_path: &Path) -> Result<Vec<DirEntry>, std::io::Error> {
  let mut paths: Vec<std::fs::DirEntry> = vec!();
  let entries = read_dir(input_path)?;
  for dir in entries {
    match dir {
      Err(why) => return Err(why),
      Ok(path) => {
        if path.path().is_dir() {
            paths.append(&mut rec_read_dir(&path.path())?);
        }
        if let Some(extension) = path.path().extension(){
          if extension == "csproj" {
            paths.push(path);
          }
        }
      }
    }
  }
  Ok(paths)
}

pub fn get_deps(paths: Vec<DirEntry>) -> Result<Vec<Deps>, std::io::Error> {
  let mut deps_vec: Vec<Deps> = vec!();
  for path in paths{
    match from_str(&read_csproj(&path.path())?) {
      Err(why) => println!("{:?}", why),
      Ok(mut proj) => {
        let deps = 
        Deps::from_proj_str(
          &path.path().file_stem().unwrap().to_os_string().into_string().unwrap(), 
          &mut proj).unwrap();
          deps_vec.push(deps);
      }
    }
  }
  Ok(deps_vec)
}

fn read_csproj(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut proj = String::new();
    file.read_to_string(&mut proj)?;

    if proj.starts_with("\u{feff}") {
      proj = proj.split_off(3);
    }
    Ok(proj)
}

