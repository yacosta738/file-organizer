use std::collections::HashMap;
use crate::globals::{DIRS, TEXT_EXT, IMAGE_EXT, AUDIO_EXT, VIDEO_EXT, COMPRESSED_EXT, EXECUTABLE_EXT};

pub fn files_extension() -> HashMap<&'static str, &'static str> {
  let mut map = HashMap::new();

  to_map(&mut map, &TEXT_EXT, DIRS[0]);
  to_map(&mut map, &IMAGE_EXT, DIRS[1]);
  to_map(&mut map, &VIDEO_EXT, DIRS[2]);
  to_map(&mut map, &AUDIO_EXT, DIRS[3]);
  to_map(&mut map, &COMPRESSED_EXT, DIRS[4]);
  to_map(&mut map, &EXECUTABLE_EXT, DIRS[5]);

  return map;
}

fn to_map(map: &mut HashMap<&'static str, &'static str>, ext: &[&'static str], dir: &'static str) {
    for ext in ext {
        map.insert(ext, dir);
    }
}

