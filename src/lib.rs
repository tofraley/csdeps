use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str};
use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::path::Path;
use std::ffi::{OsStr};

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
    fn new(in_name: &OsStr) -> Deps {
        Deps {
            name : in_name.to_os_string().into_string().unwrap(),
            dependencies : vec!(),
        }
    }
}

pub fn rec_read_dir(input_path: &Path) -> Vec<Deps> {
    let mut deps_vec: Vec<Deps> = vec!();
    match read_dir(input_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(dir_entry) => for dir in dir_entry {
            match dir {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(path) => {
                    if path.path().is_dir() {
                        deps_vec.append(&mut rec_read_dir(&path.path()));
                    }

                    if let Some(extension) = path.path().extension(){
                        if extension == "csproj" {
                            deps_vec.push(read_csproj(&path.path()));
                        }
                    }
                },
            }
        }
    }
    deps_vec
}

fn read_csproj(path: &Path) -> Deps {
    let mut file = File::open(path).unwrap();
    let mut csproj = String::new();
    file.read_to_string(&mut csproj).unwrap();
    if csproj.starts_with("\u{feff}") {
      csproj = csproj.split_off(3);
    }

    let proj: Project = from_str(&csproj).unwrap();

    let mut deps = Deps::new(path.file_stem().unwrap().clone()); 

    for item_group in proj.item_groups {
      if item_group.dependencies.len() > 0 {
        for dep in item_group.dependencies {
          deps.dependencies.push(dep);
        }
      }
    }

    deps
}
