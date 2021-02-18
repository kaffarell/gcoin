use std::{env, fs};
use leveldb::database::Database;
use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, WriteOptions, ReadOptions};

pub fn create_database(){
    let mut dir = env::current_dir().unwrap();
    dir.push("demo");

    let path_buf = dir.clone();
    fs::create_dir_all(dir).unwrap();

    let path = path_buf.as_path();
    let mut options = Options::new();
    options.create_if_missing = true;

    // Create Database
    let database = match Database::open(path, options) {
        Ok(db) => {db},
        Err(e) => {panic!("Failed to open database: {:?}", e)}
    };

    // Write to database
    let write_ops = WriteOptions::new();
    match database.put(write_ops, 1, &[1]) {
        Ok(_) => {()},
        Err(e) => {panic!("Failed to write to database: {:?}", e)}
    };

    // Read from database
    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, 1);
    match res {
        Ok(data) => {
            assert!(data.is_some());
            assert_eq!(data, Some(vec![1]));
        }
        Err(e) => {panic!("Failed to read from database: {:?}", e)}
    };

    let read_opts = ReadOptions::new();
    let mut iter = database.iter(read_opts);
    let entry = iter.next();
    assert_eq!(
        entry,
        Some((1, vec![1]))
    );
    println!("Data: {:?}", entry);
}