use serde::Deserialize;
use std::fs;
use std::io::{Cursor, Read};
use tar::Archive;
use zstd::stream::read::Decoder;

use crate::defaults::*;

///////////// Persona



#[derive(Debug,Clone)]
pub(crate) struct Persona {
  pub(crate) name:       String,
  pub(crate) prompt:     String,
  pub(crate) greeting:   String,
  pub(crate) dismissal:  String,
  pub(crate) emoji:      String,
  #[cfg(feature = "gui")]
  pub(crate) dimensions: ChatDimensions,
  #[cfg(feature = "tui")]
  pub(crate) text_image: String,
  #[cfg(any(feature = "gui",feature = "web"))]
  pub(crate) full_image: Vec<u8>,
}

#[cfg(feature = "gui")]
#[derive(Debug,Clone,Deserialize)]
pub(crate) struct ChatDimensions {
  pub(crate) chat_left:    f32, 
  pub(crate) chat_top:     f32, 
  pub(crate) chat_width:   f32, 
  pub(crate) chat_height:  f32, 
  pub(crate) input_left:   f32, 
  pub(crate) input_top:    f32, 
  pub(crate) input_width:  f32, 
  pub(crate) input_height: f32, 
}

#[derive(Deserialize)]
struct PersonaMetadata {
  name:      String,
  greeting:  String,
  dismissal: String,
  emoji:     String,
}

impl Persona {
  pub(crate) fn new(name: &str) -> Persona {
    match name.to_lowercase().as_str() {
      #[cfg(feature = "gromrik")]
      "gromrik" => Self::gromrik(),
      #[cfg(feature = "lyranis")]
      "lyranis" => Self::lyranis(),
      "commoner" => Self::commoner(),
      _         => Self::new(DEFAULT_PERSONA),
    }
  }

  pub(crate) fn commoner() -> Persona {
    Persona {
      name:       "Commoner".to_string(),
      prompt:     COMMONER_SYSTEM_PROMPT.to_string(),
      greeting:   COMMONER_GREETING.to_string(),
      dismissal:  COMMONER_DISMISSAL.to_string(),
      #[cfg(feature = "gui")]
      dimensions: ChatDimensions {
        chat_left:    555.0,
        chat_top:     35.0,
        chat_width:   215.0,
        chat_height:  500.0,
        input_left:   560.0,
        input_top:    550.0,
        input_width:  210.0,
        input_height: 30.0,
      },
      emoji:      COMMONER_EMOJI.to_string(),
      #[cfg(feature = "tui")]
      text_image: COMMONER_TEXT_IMAGE.to_string(),
      #[cfg(any(feature = "gui",feature = "web"))]
      full_image: COMMONER_FULL_IMAGE.to_vec(),
    }
  }

  #[cfg(feature = "gromrik")]
  pub(crate) fn gromrik() -> Persona {
    Persona {
      name:       "Gromrik".to_string(),
      prompt:     GROMRIK_PROMPT.to_string(),
      greeting:   GROMRIK_GREETING.to_string(),
      dismissal:  GROMRIK_DISMISSAL.to_string(),
      #[cfg(feature = "gui")]
      dimensions: ChatDimensions {
        chat_left:    555.0,
        chat_top:     35.0,
        chat_width:   215.0,
        chat_height:  500.0,
        input_left:   560.0,
        input_top:    550.0,
        input_width:  210.0,
        input_height: 30.0,
      },
      emoji:      GROMRIK_EMOJI.to_string(),
      #[cfg(feature = "tui")]
      text_image: GROMRIK_TEXT_IMAGE.to_string(),
      #[cfg(any(feature = "gui",feature = "web"))]
      full_image: GROMRIK_FULL_IMAGE.to_vec(),
    }
  }

  #[cfg(feature = "lyranis")]
  pub(crate) fn lyranis() -> Persona {
    Persona {
      name:       "Lyranis".to_string(),
      prompt:     LYRANIS_PROMPT.to_string(),
      greeting:   LYRANIS_GREETING.to_string(),
      dismissal:  LYRANIS_DISMISSAL.to_string(),
      #[cfg(feature = "gui")]
      dimensions: ChatDimensions {
        chat_left:    555.0,
        chat_top:     80.0,
        chat_width:   205.0,
        chat_height:  415.0,
        input_left:   545.0,
        input_top:    525.0,
        input_width:  175.0,
        input_height: 30.0,
      },
      emoji:      LYRANIS_EMOJI.to_string(),
      #[cfg(feature = "tui")]
      text_image: LYRANIS_TEXT_IMAGE.to_string(),
      #[cfg(any(feature = "gui",feature = "web"))]
      full_image: LYRANIS_FULL_IMAGE.to_vec(),
    }
  }

  pub(crate) fn load(persona_file: &String) -> Option<Self> {
    if persona_file.is_empty() { return None }
    let raw_bundle = match fs::read(persona_file.as_str()) {
      Ok(bundle) => bundle,
      Err(err) => {
        log::error!("Failed to read persona bundle from {}: {}",persona_file,err);
        return None;
      }
    };
    let cursor = Cursor::new(raw_bundle);
    let decompressor = match Decoder::new(cursor) {
      Ok(decompressor) => decompressor,
      Err(err) => {
        log::error!("ZSTD decompression failed for bundle {}: {}",persona_file,err);
        return None;
      }
    };
    let mut archive = Archive::new(decompressor);
    let mut metadata_json = Vec::new();
    let mut prompt = String::new();
    let mut text_image = String::new();
    let mut full_image = Vec::new();
    #[cfg(feature = "gui")]
    let mut layout_json = Vec::new();
    let entries = match archive.entries() {
      Ok(entries) => entries,
      Err(err) => {
        log::error!("Failed to read TAR entries from bundle {}: {}",persona_file,err);
        return None;
      }
    };
    for entry in entries {
      let mut entry = entry.ok()?;
      let path = entry.path().ok()?;
      let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
      match filename {
        "metadata.json"   => { entry.read_to_end(&mut metadata_json).unwrap(); }
        "prompt.tpl"      => { entry.read_to_string(&mut prompt).unwrap(); }
        "text_image.txt"  => { entry.read_to_string(&mut text_image).unwrap(); }
        "full_image.png"  => { 
          #[cfg(any(feature = "gui", feature = "web"))]
          entry.read_to_end(&mut full_image).unwrap(); 
        }
        "dimensions.json" => {
          #[cfg(feature = "gui")]
          entry.read_to_end(&mut layout_json).unwrap();
        }
        _ => {}
      }
    }
    if metadata_json.is_empty() { log::warn!("The metadata.json in bundle {} was empty or not found!",persona_file); }
    #[cfg(feature = "gui")]
    if layout_json.is_empty() { log::warn!("The dimensions.json in bundle {} was empty or not found!",persona_file); }
    let meta: PersonaMetadata = serde_json::from_slice(&metadata_json).ok()?;
    #[cfg(feature = "gui")]
    let dimensions: ChatDimensions = serde_json::from_slice(&layout_json).ok()?;
    Some(Self {
      name: meta.name,
      greeting: meta.greeting,
      dismissal: meta.dismissal,
      emoji: meta.emoji,
      prompt,
      #[cfg(feature = "gui")]
      dimensions,
      #[cfg(feature = "tui")]
      text_image,
      #[cfg(any(feature = "gui", feature = "web"))]
      full_image,
    })
  }
}
