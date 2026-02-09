use crate::{globals, utils};
use anyhow::{Context, Result};
use std::env::consts;
use std::fs;
use std::path::Path;

pub fn organize<P: AsRef<Path>>(path: P) -> Result<()> {
  let path = path.as_ref();
  let main_dir = fs::read_dir(path).context("unable to open directory for organization")?;
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();

  for entry in main_dir {
    let entry = entry.context("unable to read directory entry")?.path();
    let name = entry
      .file_name()
      .context("unable to get file name")?
      .to_str()
      .context("unable to convert file name to string")?;
    let ignore_dir = dirs.contains(&name);

    if ignore_dir {
      continue;
    }

    let ext = entry.extension();

    if let Some(extension) = ext {
      let ext_str = extension.to_str().context("unable to convert extension to string")?;
      let dir_name = map.get(ext_str).copied().unwrap_or(globals::DIRS[6]);
      let dest_dir = path.join(dir_name);
      utils::move_file(entry, dest_dir)?;
    } else if consts::OS != "windows" {
      let dest_dir = path.join(globals::DIRS[6]);
      utils::move_file(entry, dest_dir)?;
    }
  }
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::utils;
  use std::fs;
  use std::fs::File;
  use std::path::Path;

  const DIR: &str = "./.move_file";

  #[test]
  fn organize_files() {
    let path = Path::new(DIR);
    if path.exists() {
        fs::remove_dir_all(path).expect("unable to remove existing dir");
    }
    fs::create_dir(path).expect("unable to create dir");
    utils::create_dirs(path).expect("unable to create subdirs");

    let files = vec![
      "foo.txt", "foo.png", "foo.mp3", "foo.mp4", "foo.zip", "foo.exe",
      "foo.bar",
    ];

    create_files(&files);
    organize(path).expect("organize failed");

    let expect = vec![
      "Text/foo.txt",
      "Image/foo.png",
      "Audio/foo.mp3",
      "Video/foo.mp4",
      "Compressed/foo.zip",
      "Executable/foo.exe",
      "Other/foo.bar",
    ];
    assert_all(expect);

    fs::remove_dir_all(path).expect("unable to remove");
  }

  fn create_files(files: &[&str]) {
    for f in files {
      let path = Path::new(DIR).join(f);
      File::create(path).expect("unable to create file");
    }
  }

  fn assert_all(files: Vec<&str>) {
    for f in files {
      let path = Path::new(DIR).join(f);
      assert!(path.exists());
    }
  }
}
