extern crate libc;
extern crate nix;

use std::fs;
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use libc::{flock};
use nix::fcntl::{fcntl, FcntlArg};

use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::hash_set::HashSet;

struct Env {
    lock_files: Mutex<HashSet<String>>
}

impl Env {
    fn new() -> Self {
        Env { lock_files: Mutex::new(HashSet::new()) }
    }
}

struct DB<'a> {
    env: &'a Env,
    dbname: &'static str,
    lk_file: File,
}

impl<'a> DB<'a> {
    fn new(env: &'a Env, dbname: &'static str) -> Result<DB<'a>, String> {

        if !Self::create_db_dir(dbname) {
            return Err(String::from("io error"));
        }

        let path = Self::lock_file_path(dbname);
        if !env.lock_files.lock().unwrap().insert(path.clone()) {
            return Err(String::from(format!("({}) lock error", dbname)));
        }

        let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path);

        if let Ok(file) = file {
            let db = DB { env: env, dbname: dbname, lk_file: file };
            if !db.lock_file() {
                return Err(String::from(format!("({}) file lock error", dbname)));
            }

            println!("({}) new", db.dbname);
            Ok(db)
        } else {
            return Err(String::from(format!("({}) file create error", dbname)));
        }
    }

    fn create_db_dir(path: &str) -> bool {
        match fs::create_dir_all(path) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn lock_file_path(dbname: &'static str) -> String {
        format!("{}{}", dbname, "/LOCK")
    }

    fn lock_or_unlock(&self, file: &File, lock: bool) -> bool {
        let fd = file.as_raw_fd();
        let l_type = if lock { 3 /*F_WRLCK*/ } else { 2 /*F_UNLCK*/ };
        let flock = flock {
            l_type: l_type,
            l_whence: 0,
            l_start: 0,
            l_len: 0,
            l_pid: 0,
        };
        let arg = FcntlArg::F_SETLK(&flock);
        fcntl(fd, arg).is_ok()
    }

    fn lock_file(&self) -> bool {
        self.lock_or_unlock(&self.lk_file, true)
    }

    fn unlock_file(&self) -> bool {
        self.lock_or_unlock(&self.lk_file, false)
    }

    fn hoge(&self) {
        println!("({}) hoge", self.dbname)
    }
}

impl<'a> Drop for DB<'a> {
    fn drop(&mut self) {
        println!("({}) drop", self.dbname);
        self.unlock_file();
        self.env.lock_files.lock().unwrap().remove(&Self::lock_file_path(self.dbname));
    }
}

fn main() {
    let env = Arc::new(Env::new());

    for _ in 0..10 {
        let env = env.clone();
        thread::spawn(move || {
            match DB::new(&env, "hoge") {
              Ok(db) => { db.hoge(); }
              Err(msg) => { print!("{}\n", msg); }
            };
        });
    }
    println!("Hello, world!");
}
