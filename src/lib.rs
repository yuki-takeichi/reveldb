extern crate crc;
extern crate libc;
extern crate nix;

use std::fs;
use std::string::String;
use std::fs::OpenOptions;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use libc::flock;
use nix::fcntl::{fcntl, FcntlArg};

use std::sync::Mutex;
use std::collections::hash_set::HashSet;

pub mod c;
mod log;
mod memtable;
mod write_batch;
mod db;
mod skiplist;
//mod crc;

pub struct Env {
    pub lock_files: Mutex<HashSet<String>>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            lock_files: Mutex::new(HashSet::new()),
        }
    }
}

pub struct DB_<'a> {
    env: &'a Env,
    dbname: &'static str,
    lk_file: File,
    lock: Mutex<()>,
}

impl<'a> DB_<'a> {
    pub fn new(env: &'a Env, dbname: &'static str) -> Result<DB_<'a>, String> {
        if !Self::create_db_dir(dbname) {
            return Err(String::from("io error"));
        }

        let path = Self::lock_file_path(dbname);
        if !env.lock_files.lock().unwrap().insert(path.clone()) {
            return Err(String::from(format!(
                "({}) duplicate lock file error",
                dbname
            )));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path);

        if let Ok(file) = file {
            let db = DB_ {
                env: env,
                dbname: dbname,
                lk_file: file,
                lock: Mutex::new(()),
            };
            if !db.lock_file() {
                return Err(String::from(format!("({}) file lock error", dbname)));
            }

            // println!("({}) new", db.dbname);
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
        let l_type = if lock {
            3 /*F_WRLCK*/
        } else {
            2 /*F_UNLCK*/
        };
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

    pub fn hoge(&self) {
        println!("({}) hoge", self.dbname)
    }

    // pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), ()> {}

    pub fn open(&self) {
        let _ = self.lock.lock().unwrap();
    }
}

impl<'a> Drop for DB_<'a> {
    fn drop(&mut self) {
        let _ = self.lock.lock().unwrap();
        // println!("({}) drop", self.dbname);
        self.unlock_file();
        self.env
            .lock_files
            .lock()
            .unwrap()
            .remove(&Self::lock_file_path(self.dbname));
    }
}

#[cfg(test)]
mod tests {
    use super::{Env, DB_};

    #[test]
    fn file_lock() {
        let env = Env::new();
        let db = DB_::new(&env, "testdb").unwrap();
        assert!(db.lock_file());
    }

    #[test]
    fn simple_put() {
        let env = Env::new();
        let db = DB_::new(&env, "testdb").unwrap();
        // let status = db.put("hoge", "piyo");
        // assert!(status.is_ok());
    }
}
