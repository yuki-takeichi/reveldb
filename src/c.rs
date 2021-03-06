use std::slice;
use std::mem::transmute;
use std::ptr::null;
use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use libc::{c_void, free};

use write_batch::*;
use db::*;

// Struct

#[repr(u8)]
enum CompressionType {
    NoCompression,
    Snappy,
}

// C Struct

pub struct leveldb_t {
    rep: DB,
}

pub struct leveldb_env_t {}

pub struct leveldb_cache_t {}

pub struct leveldb_logger_t {}

// TODO introduce NonZero
pub struct leveldb_comparator_t {
    state: *mut c_void,
    destructor: extern "C" fn(arg: *mut c_void),
    compare: extern "C" fn(
        arg: *mut c_void,
        a: *mut char,
        alen: *mut usize,
        b: *mut char,
        blen: *mut usize,
    ) -> i32,
    name: extern "C" fn(arg: *mut c_void) -> *mut char,
}

impl Drop for leveldb_comparator_t {
    fn drop(&mut self) {
        (self.destructor)(self.state);
    }
}

// XXX #[derive(Default)]
pub struct leveldb_options_t<'a> {
    comparator: Option<&'a mut leveldb_comparator_t>,
    create_if_missing: bool,
    error_if_exists: bool,
    cache: Option<&'a mut leveldb_cache_t>,
    env: Option<&'a mut leveldb_env_t>,
    info_log: Option<&'a mut leveldb_logger_t>,
    write_buffer_size: usize,
    paranoid_checks: bool,
    max_open_files: u32,
    block_size: usize,
    block_restart_interval: u32,
    max_file_size: usize,
    compression: CompressionType,
}

// XXX #[derive(Default)]
pub struct leveldb_readoptions_t {
    verify_checksums: bool,
    fill_cache: bool,
}

// XXX #[derive(Default)]
pub struct leveldb_writeoptions_t {
    sync: bool,
}

pub struct leveldb_writebatch_t {
    rep: WriteBatch,
}

// XXX #[derive(Default)]
pub struct leveldb_iterator_t {
    // XXX make trait Iterator
    rep: DBIterator,
}

// Misc

#[no_mangle]
pub extern "C" fn leveldb_major_version() -> i8 {
    1
}

#[no_mangle]
pub extern "C" fn leveldb_minor_version() -> i8 {
    21
}

// DB

#[no_mangle]
pub extern "C" fn leveldb_open(
    options: *const leveldb_options_t,
    name: *const c_char,
    errptr: *mut *mut c_char,
) -> *mut leveldb_t {
    let options = unsafe { options.as_ref().expect("null pointer") };
    if options.create_if_missing {
        let db = Box::new(leveldb_t { rep: DB::new() });
        Box::into_raw(db)
    } else {
        let err = CString::new(format!(
            "Invalid argument: {}: does not exist (create_if_missing is false)",
            unsafe { CStr::from_ptr(name).to_string_lossy() }
        )).unwrap();

        unsafe {
            // error (alloc string from system allocator)
            *errptr = err.into_raw();
            //*errptr = strdup(err); XXX why not work?
        }
        null::<leveldb_t>() as *mut leveldb_t
    }
}

#[no_mangle]
pub extern "C" fn leveldb_close(db: *mut leveldb_t) {
    unsafe { Box::from_raw(db) };
}

#[no_mangle]
/* used to free err or dbname */
pub extern "C" fn leveldb_free(ptr: *mut c_char) {
    unsafe { free(ptr as *mut c_void) };
}

#[no_mangle]
pub extern "C" fn leveldb_get(
    db: *mut leveldb_t,
    options: *const leveldb_readoptions_t,
    key: *const c_char,
    keylen: usize,
    vallen: *mut usize,
    errptr: *mut *mut c_char,
) -> *mut c_char {
    let db = unsafe { db.as_ref().expect("null pointer") };
    let key = unsafe { slice::from_raw_parts(key, keylen) };
    match db.rep.mem_store.get(key) {
        Some(val) => {
            let mut vec = val.to_vec();
            unsafe { *vallen = vec.len() };

            // https://github.com/rust-lang/rust/issues/36284#issuecomment-244795570
            let ptr = vec.as_mut_ptr();
            ::std::mem::forget(vec);
            ptr
        }
        None => null::<c_char>() as *mut c_char,
    }
}

// Comparator

#[no_mangle]
pub extern "C" fn leveldb_comparator_create(
    state: *mut c_void,
    destructor: extern "C" fn(arg: *mut c_void),
    compare: extern "C" fn(
        arg: *mut c_void,
        a: *mut char,
        alen: *mut usize,
        b: *mut char,
        blen: *mut usize,
    ) -> i32,
    name: extern "C" fn(arg: *mut c_void) -> *mut char,
) -> *mut leveldb_comparator_t {
    let comparator = Box::new(leveldb_comparator_t {
        state: state,
        destructor: destructor,
        compare: compare,
        name: name,
    });
    Box::into_raw(comparator)
}

#[no_mangle]
pub extern "C" fn leveldb_comparator_destroy(cmp: *mut leveldb_comparator_t) {
    unsafe { Box::from_raw(cmp) };
}

// Env
#[no_mangle]
pub extern "C" fn leveldb_create_default_env() -> *mut leveldb_env_t {
    let env = Box::new(leveldb_env_t {});
    Box::into_raw(env)
}

#[no_mangle]
pub extern "C" fn leveldb_env_destroy(env: *mut leveldb_env_t) {
    unsafe { Box::from_raw(env) };
}

// Cache

#[no_mangle]
pub extern "C" fn leveldb_cache_create_lru(capacity: usize) -> *mut leveldb_cache_t {
    let cache = Box::new(leveldb_cache_t {});
    Box::into_raw(cache)
}

#[no_mangle]
pub extern "C" fn leveldb_cache_destroy(cache: *mut leveldb_cache_t) {
    unsafe { Box::from_raw(cache) };
}

// Options

#[no_mangle]
pub extern "C" fn leveldb_options_create<'a>() -> *mut leveldb_options_t<'a> {
    println!("DEBUG: leveldb_options_create");
    // TODO set the default value as original leveldb impl
    let options = Box::new(leveldb_options_t {
        comparator: None,
        create_if_missing: false,
        error_if_exists: false,
        cache: None,
        env: None,
        info_log: None,
        write_buffer_size: 4 * 1024 * 1024, // 4MB
        paranoid_checks: false,
        max_open_files: 1000,
        block_size: 4 * 1024, // 4KB
        block_restart_interval: 16,
        max_file_size: 10,
        compression: CompressionType::Snappy,
    });
    Box::into_raw(options)
}

#[no_mangle]
pub extern "C" fn leveldb_options_destroy(options: *mut leveldb_options_t) {
    unsafe { Box::from_raw(options) };
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_create_if_missing(opt: *mut leveldb_options_t, v: bool) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.create_if_missing = v;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_comparator(
    opt: *mut leveldb_options_t,
    cmp: *mut leveldb_comparator_t,
) {
    let cmp = unsafe { cmp.as_mut().expect("null pointer") };
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.comparator = Some(cmp);
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_error_if_exists(opt: *mut leveldb_options_t, v: bool) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.error_if_exists;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_cache(
    opt: *mut leveldb_options_t,
    cache: *mut leveldb_cache_t,
) {
    let cache = unsafe { cache.as_mut().expect("null pointer") };
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.cache = Some(cache);
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_env(opt: *mut leveldb_options_t, env: *mut leveldb_env_t) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    if let Some(env) = unsafe { env.as_mut() } {
        opt.env = Some(env);
    }
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_info_log(
    opt: *mut leveldb_options_t,
    l: *mut leveldb_logger_t,
) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    if let Some(l) = unsafe { l.as_mut() } {
        opt.info_log = Some(l);
    }
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_write_buffer_size(opt: *mut leveldb_options_t, s: usize) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.write_buffer_size = s;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_max_open_files(opt: *mut leveldb_options_t, n: u32) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.max_open_files = n;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_block_size(opt: *mut leveldb_options_t, n: usize) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.block_size = n;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_block_restart_interval(opt: *mut leveldb_options_t, n: u32) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.block_restart_interval = n;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_max_file_size(opt: *mut leveldb_options_t, s: usize) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.max_file_size = s;
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_compression(opt: *mut leveldb_options_t, t: u8) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    if 0 <= t && t <= 1 {
        opt.compression = unsafe { transmute(t) };
    } else {
        panic!("expect");
    }
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_paranoid_checks(opt: *mut leveldb_options_t, v: u8) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.paranoid_checks = v != 0;
}

#[no_mangle]
pub extern "C" fn leveldb_env_get_test_directory(env: *mut leveldb_env_t) -> *const c_char {
    // XXX uniqueify
    unsafe { CString::new("/tmp/reveldbtest-0").unwrap().into_raw() }
}

#[no_mangle]
pub extern "C" fn leveldb_readoptions_create() -> *mut leveldb_readoptions_t {
    let roptions = Box::new(leveldb_readoptions_t {
        verify_checksums: false,
        fill_cache: false,
    });
    Box::into_raw(roptions)
}

#[no_mangle]
pub extern "C" fn leveldb_readoptions_set_verify_checksums(
    opt: *mut leveldb_readoptions_t,
    v: bool,
) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.verify_checksums = v;
}

#[no_mangle]
pub extern "C" fn leveldb_readoptions_set_fill_cache(opt: *mut leveldb_readoptions_t, v: bool) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.fill_cache = v;
}

#[no_mangle]
pub extern "C" fn leveldb_writeoptions_create() -> *mut leveldb_writeoptions_t {
    Box::into_raw(Box::new(leveldb_writeoptions_t { sync: false }))
}

#[no_mangle]
pub extern "C" fn leveldb_writeoptions_set_sync(opt: *mut leveldb_writeoptions_t, v: bool) {
    let opt = unsafe { opt.as_mut().expect("null pointer") };
    opt.sync = v;
}

#[no_mangle]
pub extern "C" fn leveldb_destroy_db(
    options: *const leveldb_options_t,
    name: *const c_char,
    errptr: *mut *mut c_char,
) {
}

#[no_mangle]
pub extern "C" fn leveldb_put(
    db: *mut leveldb_t,
    options: *const leveldb_writeoptions_t,
    key: *const c_char,
    keylen: usize,
    val: *const c_char,
    vallen: usize,
    errptr: *mut *mut c_char,
) {
    let db = unsafe { db.as_mut().expect("null pointer") };
    let key = unsafe { slice::from_raw_parts(key, keylen) };
    let val = unsafe { slice::from_raw_parts(val, vallen) };
    db.rep.mem_store.insert(key.to_vec(), val.to_vec());
}

#[no_mangle]
pub extern "C" fn leveldb_compact_range(
    db: *mut leveldb_t,
    start_key: *const c_char,
    start_key_len: usize,
    limit_key: *const c_char,
    limit_key_len: usize,
) {
}

#[no_mangle]
// XXX why lifetime elision does not work?
pub extern "C" fn leveldb_writebatch_create<'a>() -> *mut leveldb_writebatch_t {
    Box::into_raw(Box::new(leveldb_writebatch_t {
        rep: WriteBatch::new(),
    }))
}

#[no_mangle]
pub extern "C" fn leveldb_writebatch_put(
    b: *mut leveldb_writebatch_t,
    key: *const c_char,
    keylen: usize,
    val: *const c_char,
    vallen: usize,
) {
    let b = unsafe { b.as_mut().expect("null pointer") };
    let key = unsafe { slice::from_raw_parts(key, keylen) };
    let val = unsafe { slice::from_raw_parts(val, vallen) };
    b.rep.put(key.to_vec(), val.to_vec());
}

#[no_mangle]
pub extern "C" fn leveldb_writebatch_clear(b: *mut leveldb_writebatch_t) {
    let b = unsafe { b.as_mut().expect("null pointer") };
    b.rep.clear();
}

#[no_mangle]
pub extern "C" fn leveldb_writebatch_delete(
    b: *mut leveldb_writebatch_t,
    key: *const c_char,
    keylen: usize,
) {
    let b = unsafe { b.as_mut().expect("null pointer") };
    let key = unsafe { slice::from_raw_parts(key, keylen) };
    b.rep.delete(key.to_vec());
}
#[no_mangle]
pub extern "C" fn leveldb_writebatch_destroy(b: *mut leveldb_writebatch_t) {
    unsafe { Box::from_raw(b) };
}

#[no_mangle]
pub extern "C" fn leveldb_write(
    db: *mut leveldb_t,
    woptions: *const leveldb_writeoptions_t,
    wb: *mut leveldb_writebatch_t,
    err: *mut *mut c_char,
) {
    let db = unsafe { db.as_mut().expect("null pointer") };
    let wb = unsafe { wb.as_mut().expect("null pointer") };
    /*
    wb.rep.iterate_entries(
        |key, val| {
            // XXX handle error
            db.rep.mem_store.insert(key.to_vec(), val.to_vec());
        },
        |key| {
            // XXX handle error
            db.rep.mem_store.remove(&key.to_vec());
        },
    );
    */
    wb.rep.entries.iter().for_each(|entry| match entry {
        &Entry::Deletion { ref key } => {
            db.rep.mem_store.remove(&key.to_vec());
        }
        &Entry::Value { ref key, ref val } => {
            db.rep.mem_store.insert(key.to_vec(), val.to_vec());
        }
    });
}

#[no_mangle]
pub extern "C" fn leveldb_writebatch_iterate(
    b: *mut leveldb_writebatch_t,
    state: *mut c_char,
    put: extern "C" fn(*mut c_char, k: *const c_char, klen: usize, v: *const c_char, vlen: usize),
    deleted: extern "C" fn(*mut c_char, k: *const c_char, klen: usize),
) {
    let b = unsafe { b.as_mut().expect("null pointer") };
    b.rep.iterate_entries(
        |key, val| put(state, key.as_ptr(), key.len(), val.as_ptr(), val.len()),
        |key| deleted(state, key.as_ptr(), key.len()),
    );
}

#[no_mangle]
pub extern "C" fn leveldb_create_iterator(
    db: *mut leveldb_t,
    options: *mut leveldb_readoptions_t,
) -> *mut leveldb_iterator_t {
    let db = unsafe { db.as_ref().expect("null pointer") };
    let options = unsafe { options.as_ref().expect("null pointer") };

    Box::into_raw(Box::new(leveldb_iterator_t {
        rep: db.rep.iterator(),
    }))
}

#[no_mangle]
pub extern "C" fn leveldb_iter_valid(iter: *const leveldb_iterator_t) -> bool {
    let iter = unsafe { iter.as_ref().expect("null pointer") };
    iter.rep.valid()
}

#[no_mangle]
pub extern "C" fn leveldb_iter_seek_to_first(iter: *mut leveldb_iterator_t) {
    let mut iter = unsafe { iter.as_mut().expect("null pointer") };
    iter.rep.seek_to_first();
}

#[no_mangle]
pub extern "C" fn leveldb_iter_seek_to_last(iter: *mut leveldb_iterator_t) {
    let mut iter = unsafe { iter.as_mut().expect("null pointer") };
    iter.rep.seek_to_last();
}

#[no_mangle]
pub extern "C" fn leveldb_iter_seek(iter: *mut leveldb_iterator_t, k: *const c_char, klen: usize) {
    let mut iter = unsafe { iter.as_mut().expect("null pointer") };
}

#[no_mangle]
pub extern "C" fn leveldb_iter_next(iter: *mut leveldb_iterator_t) {
    let mut iter = unsafe { iter.as_mut().expect("null pointer") };
    iter.rep.next()
}

#[no_mangle]
pub extern "C" fn leveldb_iter_prev(iter: *mut leveldb_iterator_t) {
    let mut iter = unsafe { iter.as_mut().expect("null pointer") };
    iter.rep.prev()
}

#[no_mangle]
pub extern "C" fn leveldb_iter_key(
    iter: *const leveldb_iterator_t,
    klen: *mut usize,
) -> *const c_char {
    let iter = unsafe { iter.as_ref().expect("null pointer") };
    let s = iter.rep.key();
    unsafe { *klen = s.len() };
    s.as_ptr()
}

#[no_mangle]
pub extern "C" fn leveldb_iter_value(
    iter: *const leveldb_iterator_t,
    vlen: *mut usize,
) -> *const c_char {
    let iter = unsafe { iter.as_ref().expect("null pointer") };
    let s = iter.rep.value();
    unsafe { *vlen = s.len() };
    s.as_ptr()
}

#[no_mangle]
pub extern "C" fn leveldb_iter_destroy(iter: *mut leveldb_iterator_t) {
    unsafe { Box::from_raw(iter) };
}

#[no_mangle]
pub extern "C" fn leveldb_iter_get_error(
    iter: *const leveldb_iterator_t,
    errptr: *mut *mut c_char,
) {
    let iter = unsafe { iter.as_ref().expect("null pointer") };
}

#[no_mangle]
pub extern "C" fn leveldb_debug(ptr: *const c_void) {
    println!("{:?}", ptr);
}
