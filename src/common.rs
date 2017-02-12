use std::fs;
use std::path;
use std::fmt;

use std::fs::File;
use std::io::Read;

// use plugins::{Installer};

/// A wrapper for a pair like (name, version) for any programming language
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Lang<'a> {
    /// Lang name
    pub name: &'a str,
    /// Lang version
    pub version: &'a str,
}

impl<'a> Lang<'a> {
    /// Returns a Lang instance from a string like rust=1.13.0
    ///
    /// ```
    /// let expect = Some(envirius::common::Lang{
    ///                     name: "node",
    ///                     version: "1.2.3",
    /// });
    /// let result = envirius::common::Lang::from("node=1.2.3");
    /// assert_eq!(expect, result);
    /// ```
    pub fn from(s: &str) -> Option<Lang> {
        let v: Vec<&str> = s.split("=").collect();
        if v.len() != 2 {
            return None;
        }
        Some(Lang {
            name: v[0],
            version: v[1],
        })
    }
}


/// A wrapper for a certain virtual environment
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Environment {
    /// environment's name
    name: String,
    /// environment's meta information
    meta: String,
}

impl Environment {
    /// Creates a new environment
    pub fn new(name: String, meta: String) -> Environment {
        Environment {
            name: name,
            meta: meta,
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


/// A wrapper for all environments and other envirius's enternal stuff
pub struct Nv {
    /// Root directory
    root: String,
}

impl Nv {
    /// Creates a new Nv instance + check all paths
    pub fn new(root: String) -> Nv {
        let _ = fs::create_dir_all(path::Path::new(&root));
        Nv { root: root }
    }

    pub fn get_environments(&self) -> Vec<Environment> {
        let mut envs = path::Path::new(&self.root)
            .join("envs")
            .read_dir()
            .unwrap()
            .map(|e| {
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
            })
            .collect::<Vec<_>>();
        envs.sort();
        envs
    }

    /// Prints all environments's name into STDIN
    pub fn print_environments(&self, show_meta: bool) -> () {
        let environments = self.get_environments();
        for e in &environments {
            if show_meta {
                println!("{:?}", *e);
            } else {
                println!("{}", *e);
            };
        }
    }

    /// Check is environment exists with such name
    pub fn is_exists(&self, env_name: &str) -> bool {
        for e in self.get_environments() {
            if e.name == env_name {
                return true;
            }
        }

        false
    }

    /// Removes environment with such name
    pub fn remove_env(&self, _: &str) -> bool {
        false
    }

    /// Creates a new environment
    pub fn create_env(&self, env_name: &str, langs: Vec<Option<Lang>>) -> bool {
        println!("Create environment ...");
        //        let root = path::Path::new(&self.root);
        //        let _ = fs::create_dir_all(root.join(env_name));

        for l in langs {
            if let Some(Lang { name: n, version: v }) = l {
                println!("Installing {}=={} ...", n, v);
            }
        }

        true
    }
}
