//extern crate reveldb;

use std::os::raw::c_char;

pub struct WriteBatch {
    // XXX make private
    pub entries: Vec<Entry>,
}

pub enum Entry {
    Deletion { key: Vec<c_char> },
    Value { key: Vec<c_char>, val: Vec<c_char> },
}

impl WriteBatch {
    pub fn new() -> WriteBatch {
        WriteBatch {
            entries: Vec::new(),
        }
    }

    pub fn put(&mut self, key: Vec<c_char>, val: Vec<c_char>) {
        self.entries.push(Entry::Value { key: key, val: val });
    }

    pub fn delete(&mut self, key: Vec<c_char>) {
        self.entries.push(Entry::Deletion { key: key });
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn iterate_entries<F1, F2>(&self, mut put: F1, mut deleted: F2)
    where
        F1: FnMut(&[c_char], &[c_char]),
        F2: FnMut(&[c_char]),
    {
        self.entries.iter().for_each(|entry| match entry {
            &Entry::Deletion { ref key } => deleted(&key[..]),
            &Entry::Value { ref key, ref val } => put(&key[..], &val[..]),
        });
    }
}
