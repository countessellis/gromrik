use std::{
  collections::HashMap,
  env,
  fs,
  io::{Write,stdout,stdin},
};

use crate::defaults::*;

pub(crate) fn prompt(prompt: String, default: String) -> String {
  print!("{} ",prompt);
  stdout().flush().ok();
  let mut response = String::new();
  let _ = stdin().read_line(&mut response);
  if response.trim().is_empty() { response = default };
  response.trim_end().to_string()
}

pub(crate) fn build_path(path: &String, prefix: &String) -> String {
  let prefix: String = prefix.strip_suffix('/').unwrap_or(prefix).to_string();
  let mut parts: Vec<&str> = path.split("/").collect();
  if !prefix.is_empty() {
    parts.reverse();
    parts.push(prefix.as_str());
    parts.reverse();
  }
  let directory: String = parts[..parts.len()-1].join("/");
  let file: String = parts[parts.len()-1].to_string();
  if directory.is_empty() {
    file.clone()
  } else {
    match fs::exists(&directory) {
      Ok(true) => directory+"/"+&file,
      Ok(false) => {
        match fs::create_dir_all(&directory) {
          Ok(()) => directory+"/"+&file,
          Err(_) => path.clone(),
        }
      },
      Err(_) => path.clone(),
    }
  }
}

pub(crate) fn bin_name() -> String {
  match env::current_exe() {
    Ok(path) => {
       let path = path.as_path();
       match path.file_name() {
         Some(file_name) => match file_name.to_str() {
           Some(file_name) => return file_name.to_string(),
           None => {},
         },
         None => {},
       }
    },
    Err(_) => {},
  }
  BUILD_NAME.to_string()
}

pub(crate) fn render(template: &str, fields: &HashMap<&str,String>) -> String {
  let mut prompt: String = template.to_string();
  for (key,value) in fields { 
    prompt = prompt.replace(&format!("{{{{{}}}}}",key.to_uppercase()),value);
  }
  prompt
}

