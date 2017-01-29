use libc::c_void;

// C Struct

pub struct leveldb_t {
}

pub struct leveldb_env_t {
}

pub struct leveldb_cache_t {
}

pub struct leveldb_comparator_t {
    state: *mut c_void,
    destructor: extern "C" fn(arg: *mut c_void),
    compare: extern "C" fn(arg: *mut c_void,
                           a: *mut char,
                           alen: *mut usize,
                           b: *mut char,
                           blen: *mut usize)
                           -> i32,
    name: extern "C" fn(arg: *mut c_void) -> *mut char,
}

impl Drop for leveldb_comparator_t {
    fn drop(&mut self) {
        (self.destructor)(self.state);
    }
}

pub struct leveldb_options_t<'a> {
    comparator: Option<&'a mut leveldb_comparator_t>,
    create_if_missing: bool,
}

// Misc

#[no_mangle]
pub extern "C" fn leveldb_major_version() -> i8 {
    1
}

#[no_mangle]
pub extern "C" fn leveldb_minor_version() -> i8 {
    19
}

// DB

#[no_mangle]
pub extern "C" fn leveldb_open() -> *mut leveldb_t {
    let db = Box::new(leveldb_t {});
    Box::into_raw(db)
}

#[no_mangle]
pub extern "C" fn leveldb_close(db: *mut leveldb_t) {}

#[no_mangle]
pub extern "C" fn leveldb_free(db: *mut leveldb_t) {
    unsafe {
        Box::from_raw(db);
    }
}

// Comparator

#[no_mangle]
pub extern "C" fn leveldb_comparator_create(state: *mut c_void,
                                            destructor: extern "C" fn(arg: *mut c_void),
                                            compare: extern "C" fn(arg: *mut c_void,
                                                                   a: *mut char,
                                                                   alen: *mut usize,
                                                                   b: *mut char,
                                                                   blen: *mut usize)
                                                                   -> i32,
                                            name: extern "C" fn(arg: *mut c_void) -> *mut char)
                                            -> *mut leveldb_comparator_t {
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
    unsafe {
        Box::from_raw(cmp);
    }
}

// Env
#[no_mangle]
pub extern "C" fn leveldb_create_default_env() -> *mut leveldb_env_t {
    let env = Box::new(leveldb_env_t {});
    Box::into_raw(env)
}

#[no_mangle]
pub extern "C" fn leveldb_env_destroy(env: *mut leveldb_env_t) {
    unsafe {
        Box::from_raw(env);
    }
}

// Cache

#[no_mangle]
pub extern "C" fn leveldb_cache_create_lru(capacity: usize) -> *mut leveldb_cache_t {
    let cache = Box::new(leveldb_cache_t {});
    Box::into_raw(cache)
}

#[no_mangle]
pub extern "C" fn leveldb_cache_destroy(cache: *mut leveldb_cache_t) {
    unsafe {
        Box::from_raw(cache);
    }
}

// Options

#[no_mangle]
pub extern "C" fn leveldb_options_create<'a>() -> *mut leveldb_options_t<'a> {
    let options = Box::new(leveldb_options_t {
        comparator: None,
        create_if_missing: false,
    });
    Box::into_raw(options)
}

#[no_mangle]
pub extern "C" fn leveldb_options_destroy(options: *mut leveldb_options_t) {
    unsafe {
        Box::from_raw(options);
    }
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_comparator(opt: *mut leveldb_options_t,
                                                 cmp: *mut leveldb_comparator_t) {
    let mut cmp = unsafe { Box::from_raw(cmp) };
    let mut opt = unsafe { Box::from_raw(opt) };
    opt.comparator = Some(&mut cmp);
    Box::into_raw(opt);
    // Box::into_raw(cmp);
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_error_if_exists(opt: *mut leveldb_options_t, v: u8) {
    let mut opt = unsafe { Box::from_raw(opt) };
    opt.create_if_missing = v != 0;
    Box::into_raw(opt);
}

#[no_mangle]
pub extern "C" fn leveldb_options_set_cache(opt: *mut leveldb_options_t,
                                            cache: *mut leveldb_cache_t) {
}
