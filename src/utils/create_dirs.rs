use crate::globals;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn create_dirs<P: AsRef<Path>>(dir: P, dry_run: bool) -> Result<()> {
  let dir = dir.as_ref();
  for value in globals::DIRS {
    let path = dir.join(value);

    if dry_run && !path.exists() {
      println!("[DRY RUN] Would create {} directory at {}", value, path.display());
      continue;
    }
    if !path.exists() {
      fs::create_dir(&path)
        .with_context(|| format!("Unable to create {} directory at {}", value, path.display()))?;
      println!("{} directory created", value);
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;
  use std::path::Path;

  #[test]
  fn create_directories() {
    let main_dir = "./create_dirs_test";
    fs::create_dir_all(main_dir).expect("unable to create");
    create_dirs(main_dir, false).expect("failed to create dirs");

    assert!(Path::new("./create_dirs_test/Text").exists());
    assert!(Path::new("./create_dirs_test/Image").exists());
    assert!(Path::new("./create_dirs_test/Audio").exists());
    assert!(Path::new("./create_dirs_test/Video").exists());
    assert!(Path::new("./create_dirs_test/Compressed").exists());
    assert!(Path::new("./create_dirs_test/Executable").exists());

    fs::remove_dir_all(main_dir).expect("unable to remove");
  }
}
