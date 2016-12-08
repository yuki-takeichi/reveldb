extern crate reveldb;

use reveldb::*;

// XXX: cargo runでassert!に失敗した際に
// テスト全体の実行が止まらないようにする方法を探す

// #[test]
// fn posix_file_lock() {
// let env1 = Env::new();
// let env2 = Env::new();
// let dbname = "testdb";
//
// let db1 = DB::new(&env1, dbname);
// assert!(db1.is_ok());
// let db2 = DB::new(&env2, dbname);
// assert!(db2.is_err());
// }
//

#[test]
fn duplicate_lock_file() {
    let env = Env::new();
    let dbname = "testdb";

    let db1 = DB::new(&env, dbname);
    assert!(db1.is_ok());
    let db2 = DB::new(&env, dbname);
    assert!(db2.is_err());
}
