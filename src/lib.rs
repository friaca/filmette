use std::{fs, io};
use home::home_dir;
use std::process::{Command, Stdio};
use std::path::{Path, PathBuf};
use color_thief::{self, ColorFormat};
use image::{Rgb, RgbImage};

mod fs_utils;

pub fn run(input_path: &PathBuf, output_path: &PathBuf) -> io::Result<()> {
  // Setup
  let home_dir: PathBuf = home_dir().unwrap();
  let identifier = input_path.file_stem().unwrap().to_str().unwrap();
  let frames_temp_path = Path::join(&home_dir, format!("_{identifier}_temp"));

  match fs::create_dir(&frames_temp_path) {
    Ok(_) => (),
    Err(err) => panic!("{}", err)
  };

  // Frames
  println!("Splitting frames...");

  let frames_status = 
    Command::new("ffmpeg")
      .arg("-hide_banner")
      .arg("-loglevel").arg("error")
      .arg("-i").arg(&input_path)
      .arg(Path::join(&frames_temp_path, "%04d.png"))
      .stdout(Stdio::null())
      .status()
      .expect("ffmpeg couldn't split frames");

  if !frames_status.success() {
    panic!("Something went wrong with ffmpeg");
  }

  // Histogram
  println!("Fetching colors...");

  let Ok(frames_paths) = fs_utils::list_frames_paths(&frames_temp_path) 
    // TODO: Erase folder with frames?
    else { panic!("Couldn't list frames") };

  let mut colors: Vec<color_thief::Color> = vec![];

  for entry in frames_paths {
    let bytes = fs_utils::read_bytes(&entry);
    let Ok(palette) = color_thief::get_palette(&bytes, ColorFormat::Rgb, 1, 2) else { panic!("Couldn't retrieve pallete") };
    colors.push(palette[0]);
  }
  
  //Image
  println!("Creating image...");

  let width: u32 = colors.len().try_into().unwrap();
  const HEIGHT: u32 = 1000;

  let mut image = RgbImage::new(width, HEIGHT);
  
  for x in 0..width {
    let color = colors[x as usize];
    
    for y in 0..HEIGHT {
      let rgb = Rgb::from([color.r, color.g, color.b]);
      image.put_pixel(x, y, rgb);
    }
  }

  let _ = image.save(
    PathBuf::new()
      .join(&output_path)
      .join(format!("filmette_{}.png", identifier)));

    
  // Cleaning up"
  let Ok(_) = fs_utils::delete_dir(&frames_temp_path) else {
    panic!("Couldn't delete temp files, they're located in {}, you can delete them manually", frames_temp_path.display()) };
      
  println!("Filmette created succesfully!");
  Ok(())
}