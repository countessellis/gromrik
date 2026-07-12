use std::time::SystemTime;
use chrono::{DateTime, Utc};
use std::env;
use std::fs::{self, File};
use std::path::Path;
use tar::Builder;
use zstd::stream::write::Encoder;

fn main() {
  println!("cargo:rerun-if-changed=Cargo.toml");
  println!("cargo:rerun-if-changed=resources");
  println!("cargo:rerun-if-changed=resources/gromrik");
  println!("cargo:rerun-if-changed=resources/lyranis");
  let build_name = env!("CARGO_PKG_NAME");
  println!("cargo:rustc-env=BUILD_NAME={}",build_name);
  let build_time: DateTime<Utc> = SystemTime::now().into();
  println!("cargo:rustc-env=BUILD_TIME={}",build_time.to_rfc3339());
  let build_id: i64 = build_time.timestamp_millis();
  println!("cargo:rustc-env=BUILD_ID={}",build_id);
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let resources_dir = Path::new(&manifest_dir).join("resources");
  let entries = fs::read_dir(&resources_dir).expect("Critical Error: 'resources' directory missing");
  for entry in entries {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_dir() {
      let bundle_dir = path.join("bundle");
      if bundle_dir.exists() && bundle_dir.is_dir() {
        let persona_name = path.file_name().unwrap().to_str().unwrap();
        let output_path = path.join(format!("{}.grom", persona_name));
        let output_file = File::create(&output_path).unwrap();
        let compressor = Encoder::new(output_file, 15).unwrap().auto_finish();
        let mut tar_builder = Builder::new(compressor);
        let bundle_files = fs::read_dir(&bundle_dir).unwrap();
        for file_entry in bundle_files {
          let file_entry = file_entry.unwrap();
          let file_path = file_entry.path();
          if file_path.is_file() {
            let filename = file_path.file_name().unwrap().to_str().unwrap();
            tar_builder.append_path_with_name(&file_path, filename).unwrap();
          }
        }
        tar_builder.finish().unwrap();
        let _encoder_ = tar_builder.into_inner().unwrap();
        if let Ok(relative_path) = output_path.strip_prefix(&manifest_dir) {
          println!("cargo:warning=Auto-bundled persona: {}", relative_path.display());
        } else {
          println!("cargo:warning=Auto-bundled persona: {}", output_path.display());
        }

      }
    }
  }
}
