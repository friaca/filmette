use std::path::PathBuf;
use std::fs;
use std::io;

pub fn list_frames_paths(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
  let mut entries = fs::read_dir(path)?
      .map(|res| res.map(|e| e.path()))
      .collect::<Result<Vec<_>, io::Error>>()?;
  
  entries.sort();
  Ok(entries)
}

pub fn read_bytes(path: &PathBuf) -> Vec<u8> {
  let Ok(bytes) = fs::read(&path) else { panic!("Couldn't read file {}", path.display()) };
  bytes
}

pub fn delete_dir(path: &PathBuf) -> io::Result<()> {
  fs::remove_dir_all(path)?;
  Ok(())
}