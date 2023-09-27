use std::fmt::Display;

pub const FILES_DIRECTORY: &str = "files";

pub const DATABASE_NAME: &str = "task_db";

pub const DATABASE_TREE_KEY_PREFIX: &str = "tree";
pub const DATABASE_FILES_KEY_PREFIX: &str = "files";

pub fn db_key_formatter(template: &'static str) -> impl FnOnce(&dyn Display) -> String {
  move |value| format!("{} {}", template, value)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fmt::{self, Display, Formatter};

  #[derive(Debug)]
  struct DummyType(u32);

  impl Display for DummyType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "{}", self.0)
    }
  }

  #[test]
  fn test_db_key_formatter() {
    let format_fn = db_key_formatter("template:");

    // Use the formatter on a DummyType instance
    let formatted = format_fn(&DummyType(42));

    // Check the output
    assert_eq!(formatted, "template: 42");
  }

  #[test]
  fn test_db_key_formatter_another_template() {
    let format_fn = db_key_formatter("key:");

    // Use the formatter on a DummyType instance
    let formatted = format_fn(&DummyType(123));

    // Check the output
    assert_eq!(formatted, "key: 123");
  }
}
