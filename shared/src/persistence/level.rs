use anyhow::anyhow;
use rusty_leveldb::{LdbIterator, Options, DB};

pub struct Level {
  db: DB,
}

impl Level {
  pub fn open(db_name: &str) -> anyhow::Result<Self> {
    let val = Self { db: DB::open(db_name, Options::default())? };

    Ok(val)
  }

  pub fn get(&mut self, key: &str) -> anyhow::Result<Vec<u8>> {
    self.db.get(key.as_bytes()).ok_or(anyhow!("Key not found in database"))
  }

  pub fn put(&mut self, key: &str, value: &str) -> anyhow::Result<()> {
    self.db.put(key.as_bytes(), value.as_bytes()).map_err(|e| anyhow!("Failed to put value with status {}", e))
  }

  pub fn read(&mut self) -> anyhow::Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut iter = self.db.new_iter()?;

    let mut res = vec![];
    while iter.advance() {
      let mut key = vec![];
      let mut val = vec![];

      iter.current(&mut key, &mut val);

      res.push((key, val));
    }
    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  // Setup and teardown for each test to ensure a clean environment
  fn setup(db_name: &str) {
    if let Err(_) = fs::remove_dir_all(db_name) {
      // Ignore the error if the directory doesn't exist
    }
  }

  fn teardown(db_name: &str) {
    fs::remove_dir_all(db_name).expect("Failed to clean up test database");
  }

  #[test]
  fn test_open() {
    setup("test_open");
    let _db = Level::open("test_open").expect("Failed to open database");
    teardown("test_open");
  }

  #[test]
  fn test_put_get() {
    setup("test_put_get");
    let mut db = Level::open("test_put_get").expect("Failed to open database");

    let key = "key1";
    let value = "value1";
    db.put(key, value).expect("Failed to put data");

    let retrieved_value = db.get(key).expect("Failed to get data");
    assert_eq!(retrieved_value, value.as_bytes());

    teardown("test_put_get");
  }

  #[test]
  fn test_read() {
    setup("test_read");
    let mut db = Level::open("test_read").expect("Failed to open database");

    let key1 = "key1";
    let value1 = "value1";
    db.put(key1, value1).expect("Failed to put data");

    let key2 = "key2";
    let value2 = "value2";
    db.put(key2, value2).expect("Failed to put data");

    let data = db.read().expect("Failed to read data");
    assert_eq!(data.len(), 2);

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.0.cmp(&b.0));

    assert_eq!(sorted_data[0].0, key1.as_bytes());
    assert_eq!(sorted_data[0].1, value1.as_bytes());
    assert_eq!(sorted_data[1].0, key2.as_bytes());
    assert_eq!(sorted_data[1].1, value2.as_bytes());

    teardown("test_read");
  }

  #[test]
  fn test_key_not_found() {
    setup("test_key_not_found");
    let mut db = Level::open("test_key_not_found").expect("Failed to open database");
    assert!(db.get("nonexistent_key").is_err());
    teardown("test_key_not_found");
  }
}
