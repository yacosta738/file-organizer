use crate::{globals, utils};
use anyhow::{Context, Result};
use std::env::consts;
use std::fs;
use std::path::Path;

pub fn organize<P: AsRef<Path>>(path: P, dry_run: bool) -> Result<()> {
  let path = path.as_ref();
  let main_dir = fs::read_dir(path)
    .with_context(|| format!("Unable to open directory: {}", path.display()))?;
  let map = utils::files_extension();
  let dirs = globals::DIRS.to_vec();

  for entry in main_dir {
    let entry = entry.with_context(|| "Error reading directory entry")?.path();
    let name = entry.file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("Could not get filename for {:?}", entry))?;

    let ignore_dir = dirs.contains(&name);

    if ignore_dir {
      continue;
    }

    let ext = entry.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase());

    if let Some(extension) = ext {
      let dir_name = map.get(extension.as_str());

      if let Some(d) = dir_name {
        let dest_dir = path.join(d);
        utils::move_file(&entry, dest_dir, dry_run)?;
      } else {
        let dest_dir = path.join(globals::DIRS[6]);
        utils::move_file(&entry, dest_dir, dry_run)?;
      }
    } else if consts::OS != "windows" {
      let dest_dir = path.join(globals::DIRS[6]);
      utils::move_file(&entry, dest_dir, dry_run)?;
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

  const DIR: &str = "./.move_file_test";

  #[test]
  fn organize_files() {
    if Path::new(DIR).exists() {
        fs::remove_dir_all(DIR).expect("unable to remove existing test dir");
    }
    fs::create_dir_all(DIR).expect("unable to create dir");
    utils::create_dirs(DIR, false).expect("failed to create category dirs");

    let files = vec![
      "foo.txt", "foo.png", "foo.mp3", "foo.mp4", "foo.zip", "foo.exe",
      "foo.bar",
    ];

    create_files(&files);
    organize(DIR, false).expect("organization failed");

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

    fs::remove_dir_all(DIR).expect("unable to remove");
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
      assert!(path.exists(), "File does not exist: {}", path.display());
    }
  }
}

#[cfg(test)]
mod extra_test {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    const TEST_DIR: &str = "./.case_test";

    #[test]
    fn case_insensitivity_test() {
        if Path::new(TEST_DIR).exists() {
            fs::remove_dir_all(TEST_DIR).expect("unable to remove existing test dir");
        }
        fs::create_dir_all(TEST_DIR).expect("unable to create dir");
        crate::utils::create_dirs(TEST_DIR, false).expect("failed to create category dirs");

        let files = vec!["FOO.TXT", "Bar.PnG"];
        for f in &files {
            File::create(Path::new(TEST_DIR).join(f)).expect("unable to create file");
        }

        organize(TEST_DIR, false).expect("organization failed");

        assert!(Path::new(TEST_DIR).join("Text/FOO.TXT").exists());
        assert!(Path::new(TEST_DIR).join("Image/Bar.PnG").exists());

        fs::remove_dir_all(TEST_DIR).expect("unable to remove");
    }
}
