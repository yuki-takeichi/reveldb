pub struct leveldb_t {
}

pub struct leveldb_env_t {
}

pub struct leveldb_cache_t {
}

#[no_mangle]
pub extern "C" fn leveldb_major_version() -> i8 {
    1
}

#[no_mangle]
pub extern "C" fn leveldb_minor_version() -> i8 {
    19
}

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
