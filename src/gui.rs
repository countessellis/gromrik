#![cfg(feature = "gui")]

use crossbeam_channel::{unbounded,Receiver};
use eframe::egui;

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// GUI

#[derive(Clone)]
pub(crate) struct GUI {
  pub(crate) config: Config,
  pub(crate) chat: Chat,
  pub(crate) recv: Receiver<StreamEvent>,
  pub(crate) history_lines: Vec<(String, String)>, 
  pub(crate) current_reply: String,                 
  pub(crate) is_answering:  bool,
  pub(crate) scroll_to_bottom: bool,
  pub(crate) input: String,
}

impl GUI {
  pub(crate) fn new(config: &Config) -> GUI {
    let (send, recv) = unbounded::<StreamEvent>();
    let initial_greeting = vec![(config.persona.name.clone(),config.persona.greeting.clone())];
    GUI {
      config: config.clone(),
      chat: Chat::new(&config,send),
      recv: recv,
      history_lines: initial_greeting,
      current_reply: String::new(),
      is_answering: false,
      scroll_to_bottom: false,
      input: String::new(),
    }
  }

  pub(crate) fn run(self) {
    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default()
        .with_resizable(false)          
        .with_app_id(self.config.persona.name.clone()),
      ..Default::default()
    };
    let title: String = self.config.persona.name.clone();
    let instance = self;
    let _ = eframe::run_native(
      &title,
      options,
      Box::new(|cc| {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
          "persona_gothic".to_owned(),
          std::sync::Arc::new(egui::FontData::from_owned(PERSONA_FONT.to_vec())),
        );
        fonts.font_data.insert(
          "dialogue_serif".to_owned(),
          std::sync::Arc::new(egui::FontData::from_owned(DIALOGUE_FONT.to_vec())),
        );
        fonts.families.insert(
            egui::FontFamily::Name("persona".into()),
            vec!["persona_gothic".to_owned()],
        );
        fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "dialogue_serif".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        Ok(Box::new(instance))
      }),
    );
  }
}

impl eframe::App for GUI {
 fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let mut received_tokens = false;
    while let Ok(event) = self.recv.try_recv() {
      match event {
        StreamEvent::Token(token) => {
          self.current_reply.push_str(&token);
          self.scroll_to_bottom = true;
          received_tokens = true;
        },
        StreamEvent::Finished => {
          if !received_tokens && self.current_reply.trim().is_empty() {
            log::error!("Failed to reach Ollama. Check your connection!");
            let fallback = self.config.persona.dismissal.clone();
            self.history_lines.push((self.config.persona.name.to_string(), fallback));
          } else {
            if !self.current_reply.trim().is_empty() {
              self.history_lines.push((self.config.persona.name.to_string(), self.current_reply.clone()));
            }
          }
          self.current_reply.clear();
          self.is_answering = false;
          self.scroll_to_bottom = true;
        },
      }
    }
    let image_uri = format!("bytes://persona_bg_{}.png", self.config.persona.name.clone());
    let image_source = egui::ImageSource::Bytes {
      uri: std::borrow::Cow::Owned(image_uri.clone()),
      bytes: egui::load::Bytes::from(self.config.persona.full_image.clone()),
    };
    ui.add(
      egui::Image::new(image_source).max_size(egui::vec2(ui.available_width(),ui.available_height()))
    );
    let target_rect = egui::Rect::from_min_size(
      egui::pos2(self.config.persona.dimensions.chat_left,self.config.persona.dimensions.chat_top),
      egui::vec2(self.config.persona.dimensions.chat_width,self.config.persona.dimensions.chat_height),
    );
    let ui_builder = egui::UiBuilder::new()
      .max_rect(target_rect)
      .layout(egui::Layout::top_down(egui::Align::LEFT));
    ui.scope_builder(ui_builder, |ui| {
      egui::ScrollArea::vertical()
        .max_height(self.config.persona.dimensions.chat_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
          ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
            for (sender, content) in &self.history_lines {
              let name_color = if sender == "You" {
                egui::Color32::from_rgb(100,180,220)
              } else if sender == "System" {
                egui::Color32::from_rgb(215,55,55)
              } else {
                egui::Color32::from_rgb(194,162,105)
              };
              let header_font = egui::FontId::new(16.0, egui::FontFamily::Name("persona".into()));
              ui.add(egui::Label::new(egui::RichText::new(format!("{}:", sender)).color(name_color).font(header_font)));
              ui.add_space(4.0);
              let parchment = egui::RichText::new(content.as_str()).color(egui::Color32::from_rgb(245, 235, 215));
              ui.add(egui::Label::new(parchment));
              ui.add_space(14.0);
            }
            if self.is_answering && !self.current_reply.is_empty() {
              let header_font = egui::FontId::new(16.0, egui::FontFamily::Name("persona".into()));
              ui.add(egui::Label::new(egui::RichText::new(format!("{}:", self.config.persona.name)).color(egui::Color32::from_rgb(194, 162, 105)).font(header_font)));
              ui.add_space(4.0);
              let streaming_parchment_text = egui::RichText::new(self.current_reply.as_str()).color(egui::Color32::from_rgb(245, 235, 215));
              ui.add(egui::Label::new(streaming_parchment_text));
            }
            if self.scroll_to_bottom {
              let scroll_anchor = ui.label(""); 
              scroll_anchor.scroll_to_me(Some(egui::Align::BOTTOM)); // Snap camera downward [local]
              self.scroll_to_bottom = false; // Reset state flag [local]
            }
          });
        });
    });
    let input_rect = egui::Rect::from_min_size(
      egui::pos2(self.config.persona.dimensions.input_left,self.config.persona.dimensions.input_top),
      egui::vec2(self.config.persona.dimensions.input_width,self.config.persona.dimensions.input_height),
    );
    let input_builder = egui::UiBuilder::new()
      .max_rect(input_rect)
      .layout(egui::Layout::top_down(egui::Align::LEFT));
    ui.scope_builder(input_builder, |ui| {
      if self.is_answering { ui.ctx().request_repaint(); }
      ui.style_mut().visuals.weak_text_color = Some(egui::Color32::from_rgb(180,180,185));
      let subtle_input_frame = egui::Frame::default()
        .fill(egui::Color32::from_rgba_premultiplied(25,20,15,75))
        .stroke(egui::Stroke::NONE)
        .corner_radius(egui::CornerRadius::same(4)) 
        .inner_margin(egui::Margin::symmetric(6,4)); 
      let text_edit = egui::TextEdit::singleline(&mut self.input)
        .frame(subtle_input_frame)
        .margin(egui::Margin::ZERO)
        .desired_width(self.config.persona.dimensions.input_width)
        .text_color(egui::Color32::from_rgb(245,235,215)) 
        .hint_text(format!("Ask {}...",self.config.persona.name))
        .char_limit(60);
      let response = ui.add(text_edit);
      if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        let prompt = self.input.trim().to_string();
        if !prompt.is_empty() {
          if prompt.starts_with('/') {
            let parts: Vec<&str> = prompt.split_whitespace().collect();
            let command = parts[0].to_lowercase();
            match command.as_str() {
              "/clear" => {
                self.history_lines.clear();
                self.current_reply.clear();
                self.is_answering = false;
                self.scroll_to_bottom = false;
                self.input.clear();
              },
              "/reset" => {
                *self = Self::new(&self.config.clone());
                ui.ctx().forget_all_images();
              },
              "/persona" => {
                if parts.len() > 1 {
                  let label: String = parts[1..].join(" ");
                  if let Some (persona) = self.config.personas.get(&label) {
                    let mut config: Config = self.config.clone();
                    config.persona = persona.clone();
                    *self = Self::new(&config);
                    ui.ctx().forget_all_images();
                  } else {
                    self.history_lines.push(("System".to_string(),format!("Please provide a valid persona: {}",self.config.personas.keys().cloned().collect::<Vec<String>>().join(","))));
                  }
                } else {
                  self.history_lines.push(("System".to_string(),format!("Please provide a valid persona: {}",self.config.personas.keys().cloned().collect::<Vec<String>>().join(","))));
                }
              },
              "/save" => {
                let filename: String = if parts.len() > 1 { parts[1..].join(" ") } else { self.config.history_file.clone() };
                let msg = match self.chat.save_history(&filename.to_string(), &self.history_lines) {
                  Ok(_) => format!("History successfully saved to {}.", filename),
                  Err(err) => format!("Save error: {}",err)
                };
                self.history_lines.push(("System".to_string(),msg));
                self.scroll_to_bottom = true;
                self.input.clear();
              },
              "/load" => {
                let filename: String = if parts.len() > 1 { parts[1..].join(" ") } else { self.config.history_file.clone() };
                let msg = match self.chat.load_history(&filename.to_string()) {
                  Ok(loaded_data) => {
                    self.history_lines = loaded_data;
                    format!("History loaded from {} successfully.", filename)
                  },
                  Err(e) => format!("Load error: {}", e)
                };
                self.history_lines.push(("System".to_string(), msg));
                self.scroll_to_bottom = true;
                self.input.clear();
              },
              "/exit"|"/quit" => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
              "/help" => {
                 let help_text = vec![
                   "Available Commands:",
                   "",
                   "/help  - Display this utility command list.",
                   "/clear  - Clear the dispayed chat logs completely.",
                   "/reset  - Resets the current persona.",
                   format!("/persona [persona]  - Switches the active persona, 'persona' must be one of {}.",self.config.personas.keys().cloned().collect::<Vec<String>>().join(",")).as_str(),
                   "/exit  - Safely close and exit the application.",
                   "/quit  - Also safely closes and exits the application.",
                   "/save [filename]  - Save session to a file (or configuration default).",
                   "/load [filename]  - Restore session and model context from a file.",
                   "",
                 ].join("\n");
                 self.history_lines.push(("System".to_string(), help_text));
                 self.scroll_to_bottom = true;
                 self.input.clear();
              },
              _ => {
                self.history_lines.push(("System".to_string(),format!("Unknown command: '{}'. Type /help for available commands.", command)));
                self.scroll_to_bottom = true;
                self.input.clear();
              },
            }
          } else if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close); 
          } else {
            self.history_lines.push(("You".to_string(), prompt.clone()));
            self.scroll_to_bottom = true;
            self.is_answering = true;
            if let Err(e) = self.chat.chat(&prompt) {
              log::error!("Failed to launch GUI chat pipeline thread: {}", e);
              self.is_answering = false;
            }
            self.input.clear();
          }
        }
        response.request_focus(); 
      }
    });
  }
  fn on_exit(&mut self) {
    println!("{}  {}:\n\n  {}\n",self.config.persona.emoji,self.config.persona.name,self.config.persona.dismissal);
  }
}
