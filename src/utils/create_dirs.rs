use crate::globals;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path};

pub fn create_dirs<P: AsRef<Path>>(dir: P) -> Result<()> {
  let dir = dir.as_ref();
  for value in globals::DIRS {
    let path = dir.join(value);

    if !path.exists() {
      fs::create_dir(&path).with_context(|| format!("unable to create {} directory at {}", value, path.display()))?;
      println!("{} directory created", value);
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;

  #[test]
  fn create_directories() {
    let main_dir = Path::new("./create_dirs");
    fs::create_dir(main_dir).expect("unable to create");
    create_dirs(main_dir).expect("create_dirs failed");

    assert!(main_dir.join("Text").exists());
    assert!(main_dir.join("Image").exists());
    assert!(main_dir.join("Audio").exists());
    assert!(main_dir.join("Video").exists());
    assert!(main_dir.join("Compressed").exists());
    assert!(main_dir.join("Executable").exists());
    assert!(main_dir.join("Other").exists());

    fs::remove_dir_all(main_dir).expect("unable to remove");
  }
}
