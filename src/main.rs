use std::fs;
use std::env;
use std::path;
use std::path::PathBuf;
use std::io;
use homedir;

fn main() {
  let args: Vec<String> = env::args().collect();
  
  let invocation_path: &path::Path = path::Path::new(&args[0]);
  let invocation_name = invocation_path.file_name().unwrap().to_str().unwrap();

  let home_path = homedir::my_home().unwrap().unwrap();
  let mut tower_path: path::PathBuf = home_path.to_path_buf();
  tower_path.push("the_tower/");
  
  match invocation_name {
    "seed.tower" => generate_tower(home_path, tower_path.to_owned()),
    "shift.tower" => shift_tower(tower_path.to_owned()),
    "destroy.tower" => destroy_the_tower(tower_path),
    _ => no_response(args),
  }
}

fn generate_tower(home_path: path::PathBuf, mut tower_path: path::PathBuf) {
  for i in 1..100 {
    tower_path.push(&format!("floor_{}", i));
  }
  fs::create_dir_all(tower_path).expect("add better error handling");
  let mut seed_path = home_path.to_path_buf();
  seed_path.push("seed.tower");
  let mut shift_path = home_path.to_path_buf();
  shift_path.push("the_tower/shift.tower");
  fs::rename(seed_path, shift_path).unwrap(); 
  println!("A mighty tower appears before you, it reaches into the clouds and makes you feel insignificant with its might")
}

fn shift_tower(tower_path: PathBuf) {
  let mut i = 5;
  while i > 0 {
    let mut current_floor = tower_path.to_owned();
    for j in 1..i {
      current_floor.push(format!("floor_{}", j));
    }

    let file_paths = fs::read_dir(&current_floor).unwrap();
    for file_path in file_paths {
      let file_path = file_path.unwrap();
      if !belongs_to_tower(file_path.path()) {
        let from = &file_path.path();
        let file_name = file_path.file_name();
        let mut to = current_floor.to_path_buf();
        to.push(format!("floor_{}", i));
        to.push(file_name);
        fs::rename(from, to).expect("error shifting file",);
      }
    }
    i -= 1;
  }
}

fn no_response(args: Vec<String>) {
  println!("The tower looms over you... it's cold shadow sends an eerie chill up your spine");
  dbg!(args);
}

fn belongs_to_tower(file_path: PathBuf) -> bool{
  if let Some(extension) = file_path.extension() {
    let extension = extension.to_str().unwrap();
    extension == "tower"
  } else if let Some(file_name) = file_path.file_name() {
    let file_name = file_name.to_str().unwrap();
    let name_vec: Vec<&str> = file_name.split('_').collect();
    name_vec.len() == 2 && name_vec[0] == "floor" && name_vec[1].parse::<i32>().is_ok()
  } else {
      false
  }
}

fn destroy_the_tower(tower_path: PathBuf) {
  println!("Be cautious, do you really want to eradicate the tower?");
  println!("{} <--- (y/N)?", tower_path.to_str().unwrap());
  let mut inp = String::new();
  io::stdin().read_line(&mut inp).unwrap();
  if inp.to_lowercase().trim() == "y" {
    if let Ok(_) = fs::remove_dir_all(&tower_path) {
      println!("The tower fades away leaving nothing but an empty meadow before you...")
    } else {
      println!("The tower rumbles and loses some of its items, but its weary stone walls stay tall before you.");
      println!("Something could not be removed, use rm -rf or sudo rm -rf");
    }
  } else {
    println!("You feel calm.")
  }
}