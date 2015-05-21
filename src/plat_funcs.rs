use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};
use std::env::var_os;
use std::convert::AsRef;


// THere are many things terrible with this file.  halp

#[derive(Eq, PartialEq, Clone, Copy)]
enum Platform {
    Windows,
    Linux
}

#[cfg(windows)]
const PLATFORM: Platform = Platform::Windows;
#[cfg(unix)]
const PLATFORM: Platform = Platform::Linux;

mod Windows {
    use std::path::Path;
    use std::ffi::{OsString, OsStr};
    use std::env::var_os;
    use std::convert::AsRef;

    pub fn cd<P: AsRef<Path>>(p: P) {
        println!("cd /d {}", p.as_ref().display());
    }
    pub fn setenv<S: AsRef<OsStr>, T: AsRef<OsStr>>(var: S, val: T) {
        println!("set {}={}", var.as_ref().to_str().unwrap(),
        val.as_ref().to_str().unwrap());
    }
    pub fn get_username() -> OsString {
        var_os("USERNAME").expect("Unknown username")
    }
}

mod Bash {
    use std::path::Path;
    use std::ffi::{OsString, OsStr};
    use std::env::var_os;
    use std::convert::AsRef;

    pub fn cd<P: AsRef<Path>>(p: P) {
        println!("cd '{}';", p.as_ref().display());
    }
    pub fn setenv<S: AsRef<OsStr>, T: AsRef<OsStr>>(var: S, val: T) {
        println!("export {}='{}';", var.as_ref().to_str().unwrap(),
        val.as_ref().to_str().unwrap());
    }
    pub fn get_username() -> OsString {
        var_os("USER").expect("Unknown username")
    }

}

mod Tcsh {
    use std::path::Path;
    use std::ffi::{OsString, OsStr};
    use std::env::var_os;
    use std::convert::AsRef;

    pub fn cd<P: AsRef<Path>>(p: P) {
        println!("cd '{}';", p.as_ref().display());
    }
    pub fn setenv<S: AsRef<OsStr>, T: AsRef<OsStr>>(var: S, val: T) {
        println!("setenv {} '{}';", var.as_ref().to_str().unwrap(),
        val.as_ref().to_str().unwrap());
    }
    pub fn get_username() -> OsString {
        var_os("USER").expect("Unknown username")
    }

}


enum Shell {
    Windows,
    Bash,
    Tcsh
}

pub struct Impl {
    shell: Shell
}
impl Impl {
    pub fn new() -> Impl {
        let i = if PLATFORM == Platform::Windows {
            Shell::Windows
        } else {
            match var_os("SHELL") {
                None => Shell::Bash,
                Some(oss) => match oss.to_str() {
                    Some("tcsh") => Shell::Tcsh,
                    _ => Shell::Tcsh
                }
            }
        };
        Impl {shell:i}
    }
    pub fn cd<P: AsRef<Path>>(&self, p: P) {
        match self.shell {
            Shell::Windows => Windows::cd(p),
            Shell::Bash => Bash::cd(p),
            Shell::Tcsh => Tcsh::cd(p)
        }
    }
    pub fn get_username(&self) -> OsString {
        match self.shell {
            Shell::Windows => Windows::get_username(),
            Shell::Bash => Bash::get_username(),
            Shell::Tcsh => Tcsh::get_username()
        }
    }
    pub fn setenv<S: AsRef<OsStr>, T: AsRef<OsStr>>(&self, var: S, val: T) {
        match self.shell {
            Shell::Windows => Windows::setenv(var, val),
            Shell::Bash => Bash::setenv(var, val),
            Shell::Tcsh => Tcsh::setenv(var, val)
        }
    }
}
