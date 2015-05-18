#![feature(path_ext)] 

use std::env::{home_dir, var_os, args_os};
use std::path::{Path, PathBuf, Component};
use std::fs::PathExt;
use std::fs::create_dir;
use std::convert::From;

fn print_cd(p: &Path)  {
    println!("cd '{}';", p.display());
}

fn setenv(var: &str, val: &Path) {
    println!("export {}='{}';", var, val.display());
}

fn main() {

    let username = var_os("USER").unwrap();
    let mut mytmp: PathBuf = match var_os("TMPDIR") {
        Some(s) => From::from(s),
        None => { let mut h = home_dir().expect("Unable to determine HOME directory"); h.push("tmp"); h}
    };
    if !mytmp.components().filter_map(|x| match x { Component::Normal(s) => Some(s), _ => None}).any(|x| x.to_str() == username.to_str()) {
        mytmp.push(username);
    }

    let want_new: bool = args_os().any(|ref arg| arg == "-new");

    // if a tmpdir is already set
    let prevtmp = var_os("MYTMP");
    if prevtmp.is_some() {
        let _p = prevtmp.unwrap();
        let prevtmp_path : &Path = Path::new(&_p);
        if prevtmp_path.exists() && !want_new {
            print_cd(prevtmp_path);
            return;
        }
    }

    for i in (0..99) {
        let tmp = mytmp.join(format!("{0:02}", i));
        if create_dir(&tmp).is_err() {
            continue;
        }
        print_cd(&tmp);
        setenv("MYTMP", &tmp);
        return;
    }


}
