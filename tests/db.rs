/*
extern crate reveldb;
extern crate nix;

use reveldb::*;
use nix::unistd::fork;
use nix::unistd::ForkResult::*;
use nix::sys::wait::waitpid;
use std::{process, thread, time};

#[test]
fn protect_from_different_process() {
    let dbname = "testdb";
    match fork() {
        Ok(Parent { child }) => {
            let env1 = Env::new();
            let db1 = DB::new(&env1, dbname);
            assert!(db1.is_ok());

            assert!(waitpid(child, None).is_ok());
        }
        Ok(Child) => {
            thread::sleep(time::Duration::new(1, 0));

            let env2 = Env::new();
            let db2 = DB::new(&env2, dbname);
            assert!(db2.is_err());

            process::exit(0);
        }
        Err(_) => panic!("Error: Fork Failed"),
    }
}

#[test]
fn protect_multiple_uses_from_the_same_process() {
    let env = Env::new();
    let dbname = "testdb";

    let db1 = DB::new(&env, dbname);
    assert!(db1.is_ok());
    let db2 = DB::new(&env, dbname);
    assert!(db2.is_err());
}
*/
