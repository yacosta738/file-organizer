use anyhow::{Context, Result};
use fs_extra::dir;
use std::fs;
use std::path::{Path};

pub fn move_file<P1: AsRef<Path>, P2: AsRef<Path>>(from: P1, to_dir: P2) -> Result<()> {
  let from = from.as_ref();
  let to_dir = to_dir.as_ref();
  let file_name = from.file_name().context("unable to get file name")?;
  let to = to_dir.join(file_name);

  if from.is_dir() {
    let mut options = dir::CopyOptions::new();
    options.copy_inside = true; // flattens contents into `to`
    dir::move_dir(from, &to, &options)
      .with_context(|| {
        format!(
          "Failed to move directory from '{}' to '{}'",
          from.display(),
          to.display()
        )
      })?;
  } else {
    fs::rename(from, &to).with_context(|| {
      format!(
        "Failed to rename file from '{}' to '{}'",
        from.display(),
        to.display()
      )
    })?;
  }

  println!("{}", from.display());
  println!("moved to");
  println!("{}", to.display());
  println!();

  Ok(())
}
