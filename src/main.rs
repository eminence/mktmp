extern crate setenv;


use std::env::{home_dir, var_os, args_os};
use std::path::{Path, PathBuf, Component};
use std::fs::{create_dir, create_dir_all};
use std::convert::From;
use std::ffi::OsString;


fn get_username() -> OsString {
    if cfg!(windows) {
        var_os("USERNAME").expect("Unknown username")
    } else {
        var_os("USER").expect("Unknown username")
    }
}

fn main() {
    let shell = setenv::get_shell();

    let username = get_username();
    let mut mytmp: PathBuf = match var_os("TMPDIR") {
        Some(s) => From::from(s),
        None => { let mut h = home_dir().expect("Unable to determine HOME directory"); h.push("tmp"); h}
    };
    if !mytmp.components().filter_map(|x| match x { Component::Normal(s) => Some(s), _ => None}).any(|x| x.to_str() == username.to_str()) {
        mytmp.push(username);
    }
    if !mytmp.exists() {
        create_dir_all(&mytmp).unwrap();
    }

    let want_new: bool = args_os().any(|ref arg| arg == "-new");

    // if a tmpdir is already set
    let prevtmp = var_os("MYTMP");
    if prevtmp.is_some() {
        let _p = prevtmp.unwrap();
        let prevtmp_path : &Path = Path::new(&_p);
        if prevtmp_path.exists() && !want_new {
            shell.cd(prevtmp_path);
            return;
        }
    }

    for i in 0..99 {
        let tmp = mytmp.join(format!("{0:02}", i));
        if create_dir(&tmp).is_err() {
            continue;
        }
        shell.cd(&tmp);
        shell.setenv("MYTMP", tmp);
        return;
    }


}
