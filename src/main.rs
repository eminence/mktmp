extern crate setenv;


use std::env::{home_dir, var_os, args};
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


fn mkdir(root: &Path) -> PathBuf {

    for i in 0..99 {
        let tmp = root.join(format!("{0:02}", i));
        if create_dir(&tmp).is_err() {
            continue;
        }
        return tmp;
    }
    panic!("Unable to create directory")

}

fn main() {
    let shell = setenv::get_shell();

    let username = get_username();

    let args: Vec<_> = args().collect();

    let want_new = args.len() == 2 && args[1] == "-new";

    if !want_new && args.len() == 2 {
        let path = Path::new(&args[1]);
        if path.exists() {
            let tmp = mkdir(path);
            shell.cd(&tmp);
            shell.setenv("MYTMP", tmp);
            return;
        }
    }

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

    let tmp = mkdir(&mytmp);
    shell.cd(&tmp);
    shell.setenv("MYTMP", tmp);


}
