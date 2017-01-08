use std::fs;
use std::path;
/*
struct Lang {
    name: String,
    version: String
}

struct Environment {
    name: String,
    plugins: Vec<Lang>
}
*/

pub struct EnvHome {
    path: String
}

impl EnvHome {
    pub fn list_environments(&self) -> Vec<String> {
        path::Path::new(&self.path).join("envs").read_dir().unwrap().map(|e| {
            let entry_path = e.unwrap().path();
            let file_name = entry_path.file_name().unwrap();
            let file_name_as_str = file_name.to_str().unwrap();
            String::from(file_name_as_str)
        }).collect::<Vec<String>>()
    }

    pub fn new(path: String) -> EnvHome {
        let _ = fs::create_dir_all(path::Path::new(&path));
        EnvHome {path: path}
    }
}
