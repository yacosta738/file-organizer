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
    options.copy_inside = true;
    dir::move_dir(&from, &to, &options).expect("unable to move");
  } else {
    fs::rename(&from, &to).expect("unable to move");
  }

  println!("{}", from.to_str().unwrap());
  println!("moved to");
  println!("{to}");
  println!();
}
