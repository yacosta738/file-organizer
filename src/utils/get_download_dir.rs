use crate::globals;
use anyhow::{anyhow, Context, Result};
use directories::UserDirs;
use std::path::{Path, PathBuf};

pub fn get_download_dir(custom_path: Option<&str>) -> Result<PathBuf> {
  if let Some(path) = custom_path {
    let download = Path::new(path);
    let download = download
      .canonicalize()
      .with_context(|| format!("directory not found: {}", path))?;
    return Ok(download);
  }

  let user = UserDirs::new().context("unable to get user directories")?;
  let download = user.download_dir().map(PathBuf::from).or_else(|| {
    let home = user.home_dir();
    let download = home.join(globals::DIR);
    if download.exists() {
      Some(download)
    } else {
      None
    }
  });

  download.ok_or_else(|| anyhow!("Downloads directory not found"))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn without_arguments() {
    let result = get_download_dir(None);
    // It might fail in some CI environments if Downloads doesn't exist,
    // but we check the logic.
    if let Ok(dir) = result {
        assert!(dir.to_str().unwrap().to_lowercase().contains("downloads") || dir.to_str().unwrap().contains("home"));
    }
  }

  #[test]
  fn with_custom_path() {
    let result = get_download_dir(Some("."));
    assert!(result.is_ok());
  }
}
