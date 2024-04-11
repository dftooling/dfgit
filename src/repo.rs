use std::{fs::{self, remove_dir_all, File}, io::{self, Write}, path::Path, process::Command};
use clap::builder::Str;
use serde::{Serialize, Deserialize};

use crate::template::{self, Template};


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct RepoOptions {
    template_directory: String
}

impl RepoOptions {
    pub fn default() -> RepoOptions {
        return RepoOptions {
            template_directory: String::from(".")
        };
    }
}

pub struct Repo {
    options: RepoOptions,
    remote: String,
    dir_override: Option<String>
}

impl Repo {
    pub fn clone(remote: &str) -> Repo {
        // Clone
        fs::create_dir_all(".dfgit-working").unwrap();
        Command::new("git").arg("clone").arg(remote).arg(".dfgit-working/repo").output().unwrap();
        // Read options
        let options: RepoOptions;
        if Path::new(".dfgit-working/repo/.dfgit").is_file() {
            let data = fs::read_to_string(".dfgit-working/repo/.dfgit").unwrap();
            options = serde_yaml::from_str(&data).unwrap();
        } else {
            options = RepoOptions::default();
        }
        // Return
        return Repo{options, remote: String::from(remote), dir_override: None};
    }

    pub fn set_dir_override(&mut self, dir_override: String) {
        self.dir_override = Some(dir_override);
    }

    fn get_local_template_directory(&self) -> String {
        return match self.dir_override.clone() {
            Some(str) => str.clone(),
            None => self.options.template_directory.clone()
        };
    }

    pub fn get_template_directory(&self) -> String {
        let mut dir = format!(".dfgit-working/repo/{}", self.get_local_template_directory());
        if dir.ends_with("/.") {
            dir = String::from(dir.strip_suffix("/.").unwrap());
        }
        fs::create_dir_all(&dir).unwrap();
        return dir;
    }

    pub fn read_templates(&self) -> Vec<Template> {
        let mut vec = Vec::new();
        let paths = fs::read_dir(self.get_template_directory()).unwrap();
        for path in paths {
            let ppath = path.unwrap().path();
            if ppath.extension().unwrap() == "df" {
                let spath = ppath.to_str().unwrap();
                let str = fs::read_to_string(spath).unwrap();
                let template = Template::from_json(str);
                vec.push(template);
            }
        }
        return vec;
    }

    pub fn write_templates(&self, templates: Vec<Template>) {
        let tdir = self.get_template_directory();
        // Delete existing templates
        let paths = fs::read_dir(&tdir).unwrap();
        for path in paths {
            let ppath = path.unwrap().path();
            if ppath.extension().unwrap() == "df" {
                let spath = ppath.to_str().unwrap();
                fs::remove_file(spath).unwrap();
            }
        }
        // Add templates
        for t in templates {
            let tname = t.get_filename();
            let tpath = format!("{}/{}", tdir, tname);
            let mut file = File::create(tpath).unwrap();
            file.write(t.get_json().as_bytes()).unwrap();
            file.flush().unwrap();
        }
    }

    pub fn push(&self, message: String) {
        let dir = ".dfgit-working/repo";
        let o1 = Command::new("git").arg("add").arg(&self.get_local_template_directory()).current_dir(dir).output().unwrap();
        println!("stdout: {}", String::from_utf8_lossy(&o1.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&o1.stderr));
        let o2 = Command::new("git").arg("commit").arg("-am").arg(message).current_dir(dir).output().unwrap();
        println!("stdout: {}", String::from_utf8_lossy(&o2.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&o2.stderr));
        let o3 = Command::new("git").arg("push").arg(&self.remote).current_dir(dir).output().unwrap();
        println!("stdout: {}", String::from_utf8_lossy(&o3.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&o3.stderr));
    }

    pub fn delete(&self) {
        remove_dir_all(".dfgit-working/repo").unwrap();
    }
}