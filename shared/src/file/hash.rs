use std::fs::File;
use std::io::{BufReader, Read, Result};

use crypto::digest::Digest;
use sha2::Sha256;

pub fn compute(contents: &Vec<u8>) -> String {
  let mut hasher = Sha256::new();
  hasher.update(contents);
  let result = hasher.finalize();

  format!("{:X}", result)
}

pub fn compute_for_file(file_path: &str) -> Result<String> {
  let file = File::open(file_path)?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = Vec::new();
  buf_reader.read_to_end(&mut contents)?;

  Ok(compute(&contents))
}

pub fn compute_hashes_for_files(file_paths: &[&str]) -> Result<Vec<String>> {
  let mut file_hashes = Vec::new();
  for file_path in file_paths.iter() {
    let hash = compute_for_file(file_path)?;
    file_hashes.push(hash);
  }
  Ok(file_hashes)
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_FILE_PATH: &str = "test_data/test_file.txt";
  const TEST_FILE_HASH: &str = "130C695F3F0383435B872AD3755786AB93611088C8C9C53E33EC3C026D06E9A0";

  #[test]
  fn test_compute_file_hash() {
    let hash = compute_for_file(TEST_FILE_PATH).unwrap();
    let expected_hash = TEST_FILE_HASH;
    assert_eq!(hash, expected_hash);
  }

  #[test]
  fn test_compute_hashes_for_files() {
    let hashes = compute_hashes_for_files(&[TEST_FILE_PATH]).unwrap();
    assert_eq!(hashes.len(), 1);
    // Again, replace with the actual expected hash.
    let expected_hash = TEST_FILE_HASH;
    assert_eq!(hashes[0], expected_hash);
  }
}
