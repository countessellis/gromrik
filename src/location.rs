use std::collections::HashMap;

use crate::defaults::*;

///////////// Location

#[derive(Debug,Clone)]
pub(crate) struct Location {
  pub(crate) label:      String,
  pub(crate) display:    String,
  pub(crate) scene:      String,
}

impl Location {
  pub(crate) fn new(line: &str) -> Location {
    let parts: Vec<String> = line.split("::").map(|part| part.to_string()).collect();
    Location {
      label:   parts[0].clone(),
      display: if parts.len() > 1 { parts[1].clone() } else { parts[0].clone() },
      scene:   if parts.len() > 2 { parts[2].clone() } else { parts[0].clone() },
    }
  }

  pub(crate) fn defaults() -> HashMap<String,Location> {
    Self::explore(None,LOCATION_LIST)
  }

  pub(crate) fn explore(existing: Option<&HashMap<String,Location>>, lines: &[&str]) -> HashMap<String,Location> {
    let mut locations: HashMap<String,Location> = match existing {
      Some(locations) => locations.clone(),
      None            => HashMap::new(),
    };
    for line in lines {
      let location = Self::new(line);
      locations.insert(location.label.clone(),location);
    }
    locations
  }
}
