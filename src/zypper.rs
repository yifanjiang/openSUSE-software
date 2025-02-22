use log::debug;
use std::io::prelude::*;
use std::process::{Command, Stdio};

#[derive(Clone)]
pub enum Settings {
    Enable(bool),
    Cpg(bool),
    Refresh(bool),
    Priority(i32),
}

#[derive(Clone)]
pub struct RepoInfo {
    pub id: String,
    pub alias: String,
    pub name: String,
    pub enable: bool,
    pub cpg: bool,
    pub refresh: bool,
    pub priority: i32,
    pub url: String,
}

pub struct Zypper {
    //list: Vec<RepoInfo>,
}

impl Zypper {
    //pub fn new() -> Self {
    //Self {
    //list: vec![],
    //}
    //}

    fn to_repoinfo(line: &str) -> RepoInfo {
        let r: Vec<&str> = line.split("|").collect();
        let id = r[0].trim().to_string();
        let alias = r[1].trim().to_string();
        let name = r[2].trim().to_string();
        let enable = if r[3].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let cpg = if r[4].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let refresh = if r[5].trim().to_string().contains("Yes") {
            true
        } else {
            false
        };
        let priority = r[6].trim().to_string().parse::<i32>().unwrap();
        let url = r[8].trim().to_string();

        RepoInfo {
            id: id,
            alias: alias,
            name: name,
            enable: enable,
            cpg: cpg,
            refresh: refresh,
            priority: priority,
            url: url,
        }
    }

    pub fn get_repos() -> Option<Vec<RepoInfo>> {
        let process = match Command::new("zypper")
            .arg("lr")
            .arg("-d")
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(e) => panic!("failed spawn: {}", e),
            Ok(process) => process,
        };
        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(e) => panic!("couldn't read stdout: {}", e),
            Ok(_) => {}
        }
        let v: Vec<&str> = s.split("\n").collect();
        if v.len() < 3 {
            return None;
        }
        let mut repos: Vec<RepoInfo> = vec![];
        for i in 2..v.len() - 1 {
            repos.push(Self::to_repoinfo(v[i]));
        }
        return Some(repos);
    }

    pub fn change_repo(id: String, settings: Settings) -> bool {
        let mut args: Vec<&str> = vec![];
        let mut _value = String::new();
        match settings {
            Settings::Enable(s) => args.push(if s { "-e" } else { "-d" }),
            Settings::Cpg(s) => args.push(if s { "-g" } else { "-G" }),
            Settings::Refresh(s) => args.push(if s { "-r" } else { "-n" }),
            Settings::Priority(s) => {
                args.push("-p");
                _value = s.to_string();
                args.push(&_value);
            }
        }

        let child = Command::new("pkexec")
            .arg("mod-repo")
            .args(args)
            .arg(id)
            .spawn()
            .expect("failed to run pkexec");

        let output = child.wait_with_output().expect("fail to wait pkexec");
        if !output.status.success() {
            debug!("pkexec fail");
            return false;
        } else {
            return true;
        }
    }
}

#[test]
fn get_repos() {
    Zypper::get_repos();
}
