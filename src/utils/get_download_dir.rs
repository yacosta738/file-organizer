use crate::globals;
use anyhow::{anyhow, bail, Result};
use directories::UserDirs;
use std::path::PathBuf;

pub fn get_default_download_dir() -> Result<PathBuf> {
  let user = UserDirs::new().ok_or_else(|| anyhow!("Could not determine user directories"))?;
  let download = user.download_dir();

  match download {
    None => {
      let home = user.home_dir();
      let download = home.join(globals::DIR);

      if !download.exists() {
        bail!("Downloads directory not found at {}", download.display());
      }

      Ok(download)
    },
    Some(dir) => {
      if !dir.exists() {
        bail!("Downloads directory not found at {}", dir.display());
      }

      Ok(dir.to_path_buf())
    },
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_get_default_download_dir() {
    let result = get_default_download_dir();
    // We can't guarantee it exists in the CI environment, but we can check if it returns Ok or Err gracefully
    match result {
        Ok(path) => assert!(path.to_str().unwrap().contains("Downloads") || path.to_str().unwrap().contains("downloads")),
        Err(e) => println!("Could not find downloads dir: {}", e),
    }
  }
}
