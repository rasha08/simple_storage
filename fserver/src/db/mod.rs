use std::collections::HashMap;
use std::fmt::Display;

use anyhow::anyhow;
use shared::persistence::level::Level;
use shared::tree::node::Node;

use crate::consts::{db_key_formatter, DATABASE_FILES_KEY_PREFIX, DATABASE_NAME, DATABASE_TREE_KEY_PREFIX};

pub struct DB {
  inner: Level,
}

impl DB {
  pub fn new() -> anyhow::Result<Self> {
    let inner = Level::open(DATABASE_NAME)?;
    Ok(Self { inner })
  }

  fn get_tree_key_formatter() -> impl FnOnce(&dyn Display) -> String {
    db_key_formatter(DATABASE_TREE_KEY_PREFIX)
  }

  fn get_files_key_formatter() -> impl FnOnce(&dyn Display) -> String {
    db_key_formatter(DATABASE_FILES_KEY_PREFIX)
  }

  pub fn store_tree(&mut self, tree: &Node) -> anyhow::Result<()> {
    let key = Self::get_tree_key_formatter()(&tree.hash);
    self.inner.put(&key, &serde_json::to_string(&tree)?)?;

    Ok(())
  }

  pub fn get_tree(&mut self, hash: &str) -> anyhow::Result<Node> {
    let key = Self::get_tree_key_formatter()(&hash);
    let val = self.inner.get(&key)?;

    let retrieved_string: String = String::from_utf8(val).map_err(|_| anyhow!("Failed to retrieve tree"))?;

    let tree: Node = serde_json::from_str(&retrieved_string).map_err(|_| anyhow!("Failed to decode tree"))?;

    Ok(tree)
  }

  pub fn store_files(&mut self, root_hash: &str, file_path_map: &HashMap<String, String>) -> anyhow::Result<()> {
    let key = Self::get_files_key_formatter()(&root_hash);

    self.inner.put(&key, &serde_json::to_string(&file_path_map)?)?;

    Ok(())
  }

  pub fn get_files(&mut self, root_hash: &str) -> anyhow::Result<HashMap<String, String>> {
    let key = Self::get_files_key_formatter()(&root_hash);
    let val = self.inner.get(&key)?;

    let retrieved_string: String = String::from_utf8(val).map_err(|_| anyhow!("Failed to retrieve files"))?;

    let files_map: HashMap<String, String> =
      serde_json::from_str(&retrieved_string).map_err(|_| anyhow!("Failed to decode files"))?;

    Ok(files_map)
  }
}
