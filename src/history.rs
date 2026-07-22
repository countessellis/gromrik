use std::collections::HashMap;

///////////// History

#[derive(Clone)]
pub(crate) struct Histories {
  pub(crate) persona:   String,
  pub(crate) histories: HashMap<String,History>,
  pub(crate) inputs:    Vec<String>,
}
 
#[derive(Clone)]
pub(crate) struct History {
  pub(crate) lines:   Vec<(String,String)>,
  pub(crate) display: Vec<(String,String)>,
}

impl Histories {
  pub(crate) fn new() -> Histories {
    Histories {
      persona:   String::new(),
      histories: HashMap::new(),
      inputs:    Vec::new(),
    }
  }

  pub(crate) fn clear(&mut self) {
    let display = self.display_mut();
    display.clear();
  }

  pub(crate) fn load(&mut self, lines: &Vec<(String, String)>) {
    *self.lines_mut() = lines.clone();
    *self.display_mut() = lines.clone();
  }

  pub(crate) fn switch(&mut self, persona: &String) {
    self.persona = persona.clone();
    self.histories.entry(persona.to_string()).or_insert_with(|| { History::new() } );
  }

  pub(crate) fn insert_line(&mut self, line: &(String,String)) {
    let lines   = self.lines_mut();
    lines.push(line.clone());
    let display = self.display_mut();
    display.push(line.clone());
  }

  pub(crate) fn insert_input(&mut self, value: &String) {
    self.inputs.push(value.clone());
  }

  pub(crate) fn lines(&mut self) -> &Vec<(String, String)> {
    &self.histories.entry(self.persona.clone()).or_insert_with(History::new).lines
  }

  pub(crate) fn lines_mut(&mut self) -> &mut Vec<(String, String)> {
    &mut self.histories.entry(self.persona.clone()).or_insert_with(History::new).lines
  }

  pub(crate) fn display(&mut self) -> &Vec<(String, String)> {
    &self.histories.entry(self.persona.clone()).or_insert_with(History::new).display
  }

  pub(crate) fn display_mut(&mut self) -> &mut Vec<(String, String)> {
    &mut self.histories.entry(self.persona.clone()).or_insert_with(History::new).display
  }

  pub(crate) fn inputs(&mut self) -> &Vec<String> {
    &self.inputs
  }
}

impl History {
  pub(crate) fn new() -> History {
    History {
      lines:   Vec::new(),
      display: Vec::new(),
    }
  }
}
