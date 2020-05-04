use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str};
use serde_json;
use std::fs::{File, read_dir};
use std::io::prelude::*;
use std::path::Path;
use std::ffi::{OsStr};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct PackageReference {
    #[serde(alias = "Include", default)]
    pub include: String,

    #[serde(alias = "Version", default)]
    pub version: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ItemGroup {
    #[serde(alias = "PackageReference", default)]
    pub dependencies: Vec<PackageReference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Project {
    #[serde(rename = "ItemGroup")]
    pub item_group: ItemGroup,
}

#[derive(Debug, Serialize, PartialEq)]
struct Deps {
    pub name: String,
    pub dependencies: Vec<PackageReference>,
}

impl Deps {
    fn new(in_name: &OsStr, deps: Vec<PackageReference>) -> Deps {
        Deps {
            name : in_name.to_os_string().into_string().unwrap(),
            dependencies : deps,
        }
    }
}

pub fn rec_read_dir(input_path: &Path, use_json: bool) {
    match read_dir(input_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(dir_entry) => for dir in dir_entry {
            match dir {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(path) => {
                    if path.path().is_dir() {
                        rec_read_dir(&path.path(), use_json)
                    }

                    if let Some(extension) = path.path().extension(){
                        if extension == "csproj" {
                            read_csproj(&path.path(), use_json);
                        }
                    }
                },
            }
        }
    }
}

fn read_csproj(path: &Path, use_json: bool) {

    println!("Found {:?}, attempting to read...", path);

    let mut file = File::open(path).unwrap();
    let mut csproj = String::new();
    file.read_to_string(&mut csproj).unwrap();
    if csproj.starts_with("\u{feff}") {
      csproj = csproj.split_off(3);
    }
    println!("{:?}", &csproj);

    let proj: Project = from_str(&csproj).unwrap();

    let deps = Deps::new(path.file_stem().unwrap().clone(), proj.item_group.dependencies);

    if use_json  {
        let j = serde_json::to_string_pretty(&deps).unwrap();
        println!("{}", j);
    }
    else {
        println!("Project name: {}", deps.name);
        for reference in deps.dependencies.iter() {
            println!("{} version: {}", reference.include, reference.version);
        }
    }
}
