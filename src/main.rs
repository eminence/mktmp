#![feature(path_ext)] 

use std::env::{home_dir, var_os, args_os};
use std::path::{Path, PathBuf};
use std::fs::PathExt;
use std::fs::create_dir;

fn print_cd(p: &Path)  {
    println!("cd '{}';", p.display());
}

fn setenv(var: &str, val: &Path) {
    println!("export {}='{}';", var, val.display());
}

fn main() {


    let mut mytmp: PathBuf = home_dir().expect("Unable to determine HOME directory");
    mytmp.push("tmp");

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
