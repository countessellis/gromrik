use crate::defaults::*;

///////////// Persona



#[derive(Debug,Clone)]
pub(crate) struct Persona {
  pub(crate) name:       String,
  pub(crate) prompt:     String,
  pub(crate) greeting:   String,
  pub(crate) dismissal:  String,
  pub(crate) dimensions: ChatDimensions,

  #[cfg(any(feature = "cli",feature = "tui"))]
  pub(crate) emoji:      String,
  #[cfg(feature = "tui")]
  pub(crate) text_image: String,
  #[cfg(any(feature = "gui",feature = "web"))]
  pub(crate) full_image: Vec<u8>,
}

#[derive(Debug,Clone)]
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
      #[cfg(any(feature = "cli",feature = "tui"))]
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
      #[cfg(any(feature = "cli",feature = "tui"))]
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
      #[cfg(any(feature = "cli",feature = "tui"))]
      emoji:      LYRANIS_EMOJI.to_string(),
      #[cfg(feature = "tui")]
      text_image: LYRANIS_TEXT_IMAGE.to_string(),
      #[cfg(any(feature = "gui",feature = "web"))]
      full_image: LYRANIS_FULL_IMAGE.to_vec(),
    }
  }
}
