use std::fs;
use std::path;
use std::fmt;

use std::fs::File;
use std::io::Read;

//use plugins::{Installer};

#[derive(Debug)]
pub struct Lang {
    name: String,
    version: String
}

impl Lang {
    pub fn from(s: &str) -> Option<Lang> {
        let v: Vec<&str> = s.split("=").collect();
        if v.len() != 2 {
            return None
        }
        Some(Lang{
            name: String::from(v[0]),
            version: String::from(v[1])
        })
    }
}


// Environment
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Environment {
    name: String,
    meta: String
}

impl Environment {
    pub fn new(name: String, meta: String) -> Environment {
        Environment {
            name: name,
            meta: meta
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.meta.len() > 0 {
            write!(f, "{} ({})", self.name, self.meta)
        } else {
            write!(f, "{}", self.name)
        }
    }
}


// Nv
pub struct Nv {
    root: String
}

impl Nv {
    pub fn new(root: String) -> Nv {
        let _ = fs::create_dir_all(path::Path::new(&root));
        Nv{root: root}
    }

    pub fn get_environments(&self) -> Vec<Environment> {
        let mut envs = path::Path::new(&self.root)
            .join("envs")
            .read_dir().unwrap().map(|e| {
                let dir = e.unwrap().path();
                let meta_file_path = dir.join("envirius.info");
                let mut meta_info = String::new();
                if meta_file_path.exists() {
                    let mut meta_file = File::open(meta_file_path).expect("Unable to open file");
                    meta_file.read_to_string(&mut meta_info).expect("Unable to read string");
                    meta_info.pop();
                }
                let dir_name = dir.file_name().unwrap().to_str().unwrap();
                Environment::new(String::from(dir_name), meta_info)
            }).collect::<Vec<_>>();
        envs.sort();
        envs
    }

    pub fn print_environments(&self, show_meta: bool) -> () {
        let environments = self.get_environments();
        for e in &environments {
            if show_meta {
                println!("{:?}", *e);
            } else {
                println!("{}", *e);
            };
        };
    }

    pub fn is_exists(&self, env_name: &str) -> bool {
        for e in self.get_environments() {
            if e.name == env_name {
                return true;
            }
        }

        false
    }

    pub fn remove_env(&self, _: &str) -> bool {
        false
    }

    pub fn create_env(&self, env_name: &str, langs: Vec<Option<Lang>>) -> bool {
//        let root = path::Path::new(&self.root);
//        let _ = fs::create_dir_all(root.join(env_name));

        for l in langs {
            if let Some(Lang{name: n, version: v}) = l {
                println!("Installing {}=={} ...", n, v);
            }
        }

        true
    }
}
