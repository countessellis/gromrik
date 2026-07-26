use serde::Deserialize;
use std::fs;
use std::io::{Cursor, Read};
use tar::Archive;
use zstd::stream::read::Decoder;
use std::collections::HashMap;

use crate::defaults::*;
use crate::location::*;

///////////// Persona

#[derive(Debug,Clone)]
pub(crate) struct Persona {
  pub(crate) label:      String,
  pub(crate) name:       String,
  pub(crate) prompt:     String,
  pub(crate) scene:      String,
  #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
  pub(crate) location:   String,
  #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
  pub(crate) locations:  HashMap<String,Location>,
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
  label:     String,
  name:      String,
  scene:     String,
  location:  String,
  greeting:  String,
  dismissal: String,
  emoji:     String,
}

impl Persona {
  pub(crate) fn new(label: &str) -> Persona {
    match label.to_lowercase().as_str() {
      #[cfg(feature = "gromrik")]
      "gromrik" => Self::gromrik(),
      #[cfg(feature = "lyranis")]
      "lyranis" => Self::lyranis(),
      "commoner" => Self::commoner(),
      _         => Self::new(DEFAULT_PERSONA),
    }
  }

  pub(crate) fn gather() -> HashMap<String,Persona> {
    let mut personas: HashMap<String,Persona> = HashMap::new();
    for label in PERSONA_LIST.split(",").map(|persona| persona.trim()).collect::<Vec<&str>>() {
      personas.insert(label.to_string(),Self::new(label));
    }
    #[cfg(all(feature = "gromrik", feature = "lyranis"))]
    for (index,label) in BUNDLE_LIST.iter().enumerate() {
      if let Some(persona) = Self::load(BUNDLED_PERSONAS[index]) {
        personas.insert(label.to_string(),persona);
      }
    }
    personas
  }

  pub(crate) fn commoner() -> Persona {
    Persona {
      label:      "commoner".to_string(),
      name:       "Commoner".to_string(),
      scene:      String::new(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      location:   COMMONER_LOCATION.to_string(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      locations:  Location::defaults(),
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
      label:      "gromrik".to_string(),
      name:       "Gromrik".to_string(),
      scene:      GROMRIK_SCENE.to_string(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      location:   GROMRIK_LOCATION.to_string(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      locations:  Location::explore(Some(&Location::defaults()),GROMRIK_LOCATIONS),
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
        input_top:    540.0,
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
      label:      "lyranis".to_string(),
      name:       "Lyranis".to_string(),
      scene:      LYRANIS_SCENE.to_string(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      location:   LYRANIS_LOCATION.to_string(),
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      locations:  Location::explore(Some(&Location::defaults()),LYRANIS_LOCATIONS),
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

  pub(crate) fn from_file(bundle_file: &String) -> Option<Self> {
    if bundle_file.is_empty() { return None }
    let bundle = match fs::read(bundle_file.as_str()) {
      Ok(bundle) => bundle,
      Err(err) => {
        log::error!("Failed to read persona bundle from {}: {}",bundle_file,err);
        return None;
      }
    };
    Self::load(&bundle)
  }

  pub(crate) fn load(bundle: &[u8]) -> Option<Self> {
    let cursor = Cursor::new(bundle);
    let decompressor = match Decoder::new(cursor) {
      Ok(decompressor) => decompressor,
      Err(err) => {
        log::error!("ZSTD decompression failed for bundle: {}",err);
        return None;
      }
    };
    let mut archive = Archive::new(decompressor);
    let mut metadata_json = Vec::new();
    let mut prompt: String = String::new();
    let mut text_image: String = String::new();
    #[cfg(any(feature = "gui",feature = "web"))]
    let mut full_image: Vec<u8> = Vec::new();
    #[cfg(feature = "gui")]
    let mut layout_json = Vec::new();
    #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
    let mut locations_json = Vec::new();
    let entries = match archive.entries() {
      Ok(entries) => entries,
      Err(err) => {
        log::error!("Failed to read TAR entries from bundle: {}",err);
        return None;
      }
    };
    for entry in entries {
      let mut entry = entry.ok()?;
      let path = entry.path().ok()?;
      let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
      match filename {
        "metadata.json"   => { entry.read_to_end(&mut metadata_json).unwrap(); },
        "prompt.tpl"      => { entry.read_to_string(&mut prompt).unwrap(); },
        "text_image.txt"  => { entry.read_to_string(&mut text_image).unwrap(); },
        "full_image.png"  => { 
          #[cfg(any(feature = "gui", feature = "web"))]
          entry.read_to_end(&mut full_image).unwrap(); 
        },
        "dimensions.json" => {
          #[cfg(feature = "gui")]
          entry.read_to_end(&mut layout_json).unwrap();
        },
        "locations.json"  => {
          #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
          entry.read_to_end(&mut locations_json).unwrap();
        },
        _ => {},
      }
    }
    if metadata_json.is_empty() { log::warn!("The metadata.json in bundle was empty or not found!"); }
    #[cfg(feature = "gui")]
    if layout_json.is_empty() { log::warn!("The dimensions.json in bundle was empty or not found!"); }
    #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
    if locations_json.is_empty() { log::warn!("The locations.json in bundle was empty or not found!"); }
    let meta: PersonaMetadata = serde_json::from_slice(&metadata_json).ok()?;
    #[cfg(feature = "gui")]
    let dimensions: ChatDimensions = serde_json::from_slice(&layout_json).ok()?;
    #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
    let locations: Vec<&str> = serde_json::from_slice::<Vec<&str>>(&locations_json).unwrap_or_else(|_| Vec::new());
    Some(Self {
      label:      meta.label,
      name:       meta.name,
      scene:      meta.scene,
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      location:   meta.location,
      #[cfg(any(feature = "tui", feature = "gui",feature = "web"))]
      locations:  Location::explore(Some(&Location::defaults()),&locations),
      greeting:   meta.greeting,
      dismissal:  meta.dismissal,
      emoji:      meta.emoji,
      prompt:     prompt,
      #[cfg(feature = "gui")]
      dimensions: dimensions,
      #[cfg(feature = "tui")]
      text_image: text_image,
      #[cfg(any(feature = "gui", feature = "web"))]
      full_image: full_image,
    })
  }
}
