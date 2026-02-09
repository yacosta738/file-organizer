use anyhow::{Context, Result};
use fs_extra::dir;
use std::fs;
use std::path::Path;

pub fn move_file<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q, dry_run: bool) -> Result<()> {
  let from = from.as_ref();
  let to = to.as_ref();

  let file_name = from.file_name()
    .with_context(|| format!("Could not get filename from {}", from.display()))?;
  let to_full_path = to.join(file_name);

  if dry_run {
    println!("[DRY RUN] Would move {} to {}", from.display(), to_full_path.display());
    return Ok(());
  }
  if from.is_dir() {
    let mut options = dir::CopyOptions::new();
    options.copy_inside = true;  // flattens contents into `to`
    dir::move_dir(from, &to_full_path, &options)
      .map_err(|e| {
        anyhow::anyhow!(
          "Failed to move directory from '{}' into '{}': {}",
          from.display(),
          to_full_path.display(),
          e
        )
      })?;
  } else {
    fs::rename(from, &to_full_path)
      .with_context(|| {
        format!(
          "Failed to rename file from '{}' to '{}'",
          from.display(),
          to_full_path.display()
        )
      })?;
  }

  println!("{} moved to {}", from.display(), to_full_path.display());
  Ok(())
}
