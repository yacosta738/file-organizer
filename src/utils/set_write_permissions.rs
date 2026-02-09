use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// On Unix, `set_readonly(false)` sets the permissions to `0o666` (read/write for all),
// which is not ideal. However, for the purpose of this script, which is to organize the user's "Downloads" folder,
// simply making the files writable is probably sufficient and less likely to cause issues.
#[allow(clippy::permissions_set_readonly_false)]
pub fn set_write_permissions<P: AsRef<Path>>(path: P, dry_run: bool) -> Result<()> {
  let path = path.as_ref();
  if dry_run {
    println!("[DRY RUN] Would set write permissions for files in {}", path.display());
    return Ok(());
  }
  let dir = fs::read_dir(path)
    .with_context(|| format!("Unable to open directory for permissions: {}", path.display()))?;

  for entry in dir {
    let entry = entry.with_context(|| "Error reading directory entry")?.path();
    let mt = fs::metadata(&entry)
      .with_context(|| format!("Unable to get metadata for {}", entry.display()))?;
    let mut perms = mt.permissions();
    perms.set_readonly(false);
    fs::set_permissions(&entry, perms)
      .with_context(|| format!("Unable to set permissions for {}", entry.display()))?;
  }
  Ok(())
}
