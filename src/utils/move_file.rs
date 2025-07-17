use fs_extra::dir;
use std::fs;
use std::path::PathBuf;

pub fn move_file(from: PathBuf, to: String) {
  let file_name = from.file_name().unwrap();
  let file_name = file_name.to_os_string();
  let file_name = file_name.to_str().unwrap();
  let to = format!("{to}/{file_name}");

  if from.is_dir() {
    let mut options = dir::CopyOptions::new();
    options.copy_inside = true;  // flattens contents into `to`
    dir::move_dir(&from, &to, &options)
      .map_err(|e| {
        format!(
          "Failed to move directory from '{}' into '{}': {}",
          from.display(),
          to.display(),
          e
        )
      })
      .expect("directory move failed");
  } else {
    fs::rename(&from, &to)
      .map_err(|e| {
        format!(
          "Failed to rename file from '{}' to '{}': {}",
          from.display(),
          to.display(),
          e
        )
      })
      .expect("file rename failed");
  }

  println!("{}", from.to_str().unwrap());
  println!("moved to");
  println!("{to}");
  println!();
}
