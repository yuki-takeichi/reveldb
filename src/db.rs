use std::os::raw::c_char;
use std::collections::HashMap;

pub struct DB {
    pub mem_store: HashMap<Vec<c_char>, Vec<c_char>>,
}
pub struct DBIterator {
    initialized: bool,
}

impl DB {
    pub fn new() -> DB {
        DB {
            mem_store: HashMap::new(),
        }
    }

    pub fn iterator(&self) -> DBIterator {
        DBIterator { initialized: false }
    }
}

impl DBIterator {
    pub fn valid(&self) -> bool {
        self.initialized
    }
    pub fn seek_to_first(&mut self) {
        self.initialized = true;
    }
    pub fn seek_to_last(&mut self) {
        // XXX impl
    }
    pub fn next(&mut self) {}
    pub fn prev(&mut self) {}
    pub fn key(&self) -> &[c_char] {
        unsafe { &*(&b"hoge"[..] as *const [u8] as *const [i8]) }
    }
    pub fn value(&self) -> &[c_char] {
        unsafe { &*(&b"fuga"[..] as *const [u8] as *const [i8]) }
    }
}
