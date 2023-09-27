use std::fmt;

use anyhow::anyhow;
use crypto::digest::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Position {
  Left,
  Right,
}

impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Position::Left => write!(f, "L"),
      Position::Right => write!(f, "R"),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
  pub hash: String,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}

impl Node {
  fn new(hash: String) -> Self {
    Node { hash, left: None, right: None }
  }

  fn compute_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:X}", result) //GenericArray implements std::fmt::UpperHex
  }

  pub fn create_tree(file_hashes: Vec<String>) -> anyhow::Result<Node> {
    if file_hashes.is_empty() {
      return Err(anyhow!("File Hashes List - cannot be empty"));
    }

    let mut nodes: Vec<Node> = file_hashes.into_iter().map(Node::new).collect();

    while nodes.len() > 1 {
      let mut parent_nodes = Vec::new();

      for chunk in nodes.chunks(2) {
        let node = match chunk {
          [left_node, right_node] => {
            let combined_hash = format!("{}{}", &left_node.hash, &right_node.hash);
            let parent_hash = Node::compute_hash(&combined_hash);
            let mut node = Node::new(parent_hash);
            node.left = Some(Box::new(left_node.clone()));
            node.right = Some(Box::new(right_node.clone()));
            node
          }
          [left_node] => left_node.clone(),
          _ => unreachable!(),
        };

        parent_nodes.push(node);
      }

      nodes = parent_nodes;
    }

    nodes.into_iter().next().ok_or(anyhow!("Unexpected error in creating tree"))
  }

  pub fn generate_proof(&self, hash: &str) -> Option<Vec<(String, Position)>> {
    if self.hash == hash {
      return Some(Vec::new());
    }

    if let Some(ref left_node) = self.left {
      let mut left_proof = Vec::new();
      if let Some(ref right_node) = self.right {
        left_proof.push((right_node.hash.clone(), Position::Right)); // push sibling's hash
        if let Some(mut result_proof) = left_node.generate_proof(hash) {
          result_proof.extend(left_proof);
          return Some(result_proof);
        }
      }

      let mut right_proof = Vec::new();
      right_proof.push((left_node.hash.clone(), Position::Left)); // push sibling's hash
      if let Some(mut result_proof) = self.right.as_ref()?.generate_proof(hash) {
        result_proof.extend(right_proof);
        return Some(result_proof);
      }
    }

    None
  }

  pub fn verify_proof(file_hash: &str, proof: &[(String, Position)], root_hash: &str) -> bool {
    let mut current_hash = file_hash.to_string();

    for (sibling_hash, position) in proof.iter() {
      let combined_data = match position {
        Position::Left => format!("{}{}", sibling_hash, current_hash),
        Position::Right => format!("{}{}", current_hash, sibling_hash),
      };

      current_hash = Node::compute_hash(&combined_data);
    }

    current_hash == *root_hash
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_node() {
    let node = Node::new("hash123".to_string());
    assert_eq!(node.hash, "hash123");
    assert!(node.left.is_none());
    assert!(node.right.is_none());
  }

  #[test]
  fn test_compute_hash() {
    let data = "test";
    let hash = Node::compute_hash(data);
    let expected_hash = "9F86D081884C7D659A2FEAA0C55AD015A3BF4F1B2B0B822CD15D6C15B0F00A08";
    assert_eq!(hash, expected_hash);
  }

  #[test]
  fn test_create_tree() {
    let hashes = vec!["hash1".to_string(), "hash2".to_string()];
    let tree = Node::create_tree(hashes).unwrap();
    let expected_hash = Node::compute_hash("hash1hash2");
    assert_eq!(tree.hash, expected_hash);
  }

  #[test]
  fn test_generate_proof() {
    // Setup a simple Merkle tree
    let hashes = vec!["hash1".to_string(), "hash2".to_string()];
    let tree = Node::create_tree(hashes).unwrap();

    // Generate proof for one of the hashes
    let proof = tree.generate_proof("hash2").unwrap();

    // Expect the proof to contain the sibling hash and its position
    assert_eq!(proof.len(), 1);
    assert_eq!(proof[0].1, Position::Left);
  }

  #[test]
  fn test_verify_proof() {
    // Setup a simple Merkle tree
    let hashes = vec!["hash1".to_string(), "hash2".to_string()];

    let tree = Node::create_tree(hashes).unwrap();

    // Generate proof for one of the hashes
    let proof = tree.generate_proof("hash1").unwrap();

    // Verify the proof against the Merkle root
    let is_valid = Node::verify_proof("hash1", &proof, &tree.hash);

    // Expect the proof to be valid
    assert!(is_valid);
  }

  #[test]
  fn test_invalid_proof() {
    // Setup a simple Merkle tree
    let hashes = vec!["hash1".to_string(), "hash2".to_string()];
    let tree = Node::create_tree(hashes).unwrap();

    // Generate proof for one of the hashes
    let mut proof = tree.generate_proof("hash2").unwrap();

    // Alter the proof to make it invalid
    proof[0].0 = Node::compute_hash("fake_data");

    // Verify the proof against the Merkle root
    let is_valid = Node::verify_proof("hash2", &proof, &tree.hash);

    // Expect the proof to be invalid
    assert!(!is_valid);
  }
}
