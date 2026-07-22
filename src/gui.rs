#![cfg(feature = "gui")]

use crossbeam_channel::{unbounded,Receiver};
use eframe::egui;

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// GUI

#[derive(Clone)]
pub(crate) struct GUI {
  pub(crate) config:           Config,
  pub(crate) chat:             Chat,
  pub(crate) recv:             Receiver<StreamEvent>,
  pub(crate) history_lines:    Vec<(String, String)>, 
  pub(crate) input_history:    Vec<String>,
  pub(crate) input_index:      usize,
  pub(crate) current_reply:    String,                 
  pub(crate) is_answering:     bool,
  pub(crate) is_first_frame:   bool,
  pub(crate) boot_time:        std::time::Instant,
  pub(crate) scroll_to_bottom: bool,
  pub(crate) input:            String,
}

impl GUI {
  pub(crate) fn new(config: &Config) -> GUI {
    let (send, recv) = unbounded::<StreamEvent>();
    let initial_greeting = vec![(config.persona.name.clone(),config.persona.greeting.clone())];
    let mut chat: Chat = Chat::new(&config,send);
    if !config.scene.is_empty() {
      chat.set_scene(&config.scene);
    } else if !config.persona.scene.is_empty() {
      chat.set_scene(&config.persona.scene);
    }
    GUI {
      config:           config.clone(),
      chat:             chat,
      recv:             recv,
      history_lines:    initial_greeting,
      input_history:    Vec::new(),
      input_index:      0,
      current_reply:    String::new(),
      is_answering:     false,
      is_first_frame:   true,
      boot_time:        std::time::Instant::now(),
      scroll_to_bottom: false,
      input:            String::new(),
    }
  }

  pub(crate) fn run(self) {
    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_min_inner_size([800.0, 600.0])
        .with_max_inner_size([800.0, 600.0])
        .with_resizable(false)
        .with_maximize_button(false)
        .with_decorations(false) 
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

impl GUI {
  fn splash(&mut self, ui: &mut egui::Ui) -> Option<u8> {
    let total_time = self.boot_time.elapsed();
    if total_time < std::time::Duration::from_secs(6) {
      ui.ctx().request_repaint();
      let screen_rect = ui.ctx().input(|i| i.raw.screen_rect).unwrap_or_else(|| ui.max_rect());
      let splash_builder = egui::UiBuilder::new().max_rect(screen_rect).layer_id(egui::LayerId::background());
      let mut splash_ui = ui.new_child(splash_builder);
      let image_uri = format!("bytes://splash_{}.png", self.config.persona.name.clone());
      let image_source = egui::ImageSource::Bytes {
        uri: std::borrow::Cow::Owned(image_uri.clone()),
        bytes: egui::load::Bytes::from(FULL_SPLASH),
      };
      let mut splash_image = egui::Image::new(image_source).max_size(screen_rect.size());
      let mut alpha = 255u8;
      if total_time > std::time::Duration::from_secs(4) {
        let pct = ((6.0-total_time.as_secs_f32())/2.0).clamp(0.0, 1.0);
        alpha = (pct * 255.0) as u8;
        splash_image = splash_image.tint(egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha));
      }
      splash_ui.add(splash_image);
      return Some(alpha)
    }
    None
  }
}

impl eframe::App for GUI {
  fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
    let alpha = match self.splash(ui) {
      Some(255) => return,
      Some(alpha) => 255-alpha,
      None => 255,
    };
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
              log::debug!("Response from Ollama completed: \n\n{}\n",self.current_reply);
              self.history_lines.push((self.config.persona.name.to_string(), self.current_reply.clone()));
            }
          }
          self.current_reply.clear();
          self.is_answering = false;
          self.scroll_to_bottom = true;
        },
      }
    }
    let theme_color = egui::Color32::from_rgb(130,108,72);
    ui.visuals_mut().selection.stroke = egui::Stroke::new(1.5,theme_color); 
    let bevel_frame = egui::Frame::new().inner_margin(1.0).stroke(egui::Stroke::new(2.5,theme_color));
    bevel_frame.show(ui, |ui| {
      let image_uri = format!("bytes://persona_bg_{}.png", self.config.persona.name.clone());
      let image_source = egui::ImageSource::Bytes {
        uri: std::borrow::Cow::Owned(image_uri.clone()),
        bytes: egui::load::Bytes::from(self.config.persona.full_image.clone()),
      };
      let chat_bg_tint = egui::Color32::from_rgba_unmultiplied(255,255,255,alpha);
      let image_widget = egui::Image::new(image_source).max_size(egui::vec2(ui.available_width(), ui.available_height())).tint(chat_bg_tint).show_loading_spinner(false).alt_text("");
      let image_resp = ui.add(image_widget);
      if image_resp.interact(egui::Sense::drag()).dragged() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
      }
      if alpha == 255 {
        let target_rect = egui::Rect::from_min_size(
          egui::pos2(self.config.persona.dimensions.chat_left,self.config.persona.dimensions.chat_top),
          egui::vec2(self.config.persona.dimensions.chat_width,self.config.persona.dimensions.chat_height),
        );
        let ui_builder = egui::UiBuilder::new().max_rect(target_rect).layout(egui::Layout::top_down(egui::Align::LEFT));
        let mut scroll_delta = 0.0;
        ui.input(|input| {
          if input.key_pressed(egui::Key::PageUp) {
            scroll_delta = 100.0;
          } else if input.key_pressed(egui::Key::PageDown) {
            scroll_delta = -100.0;
          }
        });
        ui.scope_builder(ui_builder, |ui| {
          let scroll_view_id = egui::Id::new("chat_scroll_viewport");
          let mut scroll_delta = 0.0;
          ui.input(|input| {
            if input.key_pressed(egui::Key::PageUp) {
              scroll_delta = 100.0;
            } else if input.key_pressed(egui::Key::PageDown) {
              scroll_delta = -100.0;
            }
          });
          egui::ScrollArea::vertical()
            .id_salt(scroll_view_id)
            .max_height(self.config.persona.dimensions.chat_height)
            .auto_shrink([false, false])
            .show(ui, |ui| {
              ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                if scroll_delta != 0.0 {
                  ui.scroll_with_delta(egui::vec2(0.0, scroll_delta));
                  self.scroll_to_bottom = false;
                }
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
                if self.scroll_to_bottom && scroll_delta == 0.0 {
                  let scroll_anchor = ui.label(""); 
                  scroll_anchor.scroll_to_me(Some(egui::Align::BOTTOM));
                  self.scroll_to_bottom = false;
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
            .char_limit(200)
            .lock_focus(true);
          let response = ui.add(text_edit);
          if self.is_first_frame && alpha == 255 {
            response.request_focus();
            self.is_first_frame = false;
          }
          if response.has_focus() {
            let tab_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::Tab));
            let up_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowUp));
            let down_pressed = ui.ctx().input(|i| i.key_pressed(egui::Key::ArrowDown));
            if tab_pressed {
              ui.ctx().memory_mut(|mem| mem.request_focus(response.id));
              let commands = vec![ "/clear","/reset","/persona ","/scene ","/save ","/load ","/exit","/quit","/help"];
              if self.input.starts_with('/') {
                let current_input = self.input.to_lowercase();
                if current_input.starts_with("/persona ") {
                  let prefix = &self.input["/persona ".len()..];
                  let prefix_lower = prefix.to_lowercase();
                  let mut persona_names: Vec<String> = self.config.personas.keys().cloned().collect();
                  persona_names.sort();
                  let matches: Vec<&String> = persona_names.iter().filter(|name| name.to_lowercase().starts_with(&prefix_lower)).collect();
                  if !matches.is_empty() {
                    let current_index = matches.iter().position(|name| **name == prefix);
                    let next_match = match current_index {
                      Some(idx) => matches[(idx + 1) % matches.len()],
                      None => matches[0],
                    };
                    self.input = format!("/persona {}", next_match);
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                      state.cursor.set_char_range(Some(egui::text::CCursorRange::one(
                        egui::text::CCursor::new(self.input.chars().count())
                      )));
                      egui::TextEdit::store_state(ui.ctx(), response.id, state);
                    }
                  }
                }
                else if current_input.starts_with("/load ") {
                  let argument = &self.input["/load ".len()..];
                  if argument.trim().is_empty() {
                    self.input = format!("/load {}", self.config.history_file);
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                      state.cursor.set_char_range(Some(egui::text::CCursorRange::one(
                        egui::text::CCursor::new(self.input.chars().count())
                      )));
                      egui::TextEdit::store_state(ui.ctx(), response.id, state);
                    }
                  }
                }
                else if current_input.starts_with("/save ") {
                  let argument = &self.input["/save ".len()..];
                  if argument.trim().is_empty() {
                    self.input = format!("/save {}", self.config.history_file);
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) {
                      state.cursor.set_char_range(Some(egui::text::CCursorRange::one(
                        egui::text::CCursor::new(self.input.chars().count())
                      )));
                      egui::TextEdit::store_state(ui.ctx(), response.id, state);
                    }
                  }
                }
                else if self.input.chars().count() > 1 {
                  let is_already_exact_command = commands.iter().any(|cmd| current_input.starts_with(*cmd));
                  if !is_already_exact_command {
                    if let Some(matched_command) = commands.iter().find(|cmd| cmd.starts_with(&current_input)) {
                      self.input = matched_command.to_string();
                      if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(),response.id) {
                        state.cursor.set_char_range(Some(egui::text::CCursorRange::one(
                          egui::text::CCursor::new(self.input.chars().count())
                        )));
                        egui::TextEdit::store_state(ui.ctx(),response.id,state);
                      }
                    }
                  }
                }
              }
            }
            if up_pressed {
              if !self.input_history.is_empty() && self.input_index > 0 {
                self.input_index = self.input_index.saturating_sub(1);
                self.input = self.input_history[self.input_index].clone();
              }
            } else if down_pressed {
              if !self.input_history.is_empty() {
                self.input_index = self.input_index.saturating_add(1);
                if self.input_index < self.input_history.len() {
                  self.input = self.input_history[self.input_index].clone();
                } else {
                  self.input_index = self.input_history.len();
                  self.input.clear();
                }
              }
            }
          }
          if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let prompt = self.input.trim().to_string();
            if !prompt.is_empty() {
              self.input_history.push(prompt.clone());
              self.input_index = self.input_history.len();
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
                  "/scene" => {
                    if parts.len() > 1 {
                      let scene: String = parts[1..].join(" ");
                      self.chat.set_scene(&scene);
                      self.history_lines.push(("System".to_string(),format!("Scene has been set to: {}",scene)));
                      self.input.clear();
                    } else {
                      self.history_lines.push(("System".to_string(),format!("Please provide a scene.")));
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
                       "/scene  - Sets the scene for the conversation, argument should be a single sentence.",
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
    });
    let r = ui.ctx().input(|i| i.raw.screen_rect).unwrap_or_else(|| ui.max_rect()); 
    let top_painter = ui.ctx().layer_painter(ui.layer_id());
    let top_left_highlight  = egui::Color32::from_rgba_unmultiplied(180,155,115,alpha); 
    let top_right_midtone   = egui::Color32::from_rgba_unmultiplied(130,108,72,alpha);  
    let bottom_left_midtone = egui::Color32::from_rgba_unmultiplied(115,95,62,alpha);   
    let bottom_right_shadow = egui::Color32::from_rgba_unmultiplied(85,70,45,alpha);    
    let stroke_width = 2.5;
    top_painter.line_segment([r.left_top(),r.right_top()],egui::Stroke::new(stroke_width,top_left_highlight));
    top_painter.line_segment([r.right_top(),r.left_top()],egui::Stroke::new(stroke_width,top_right_midtone));
    top_painter.line_segment([r.right_top(),r.right_bottom()],egui::Stroke::new(stroke_width,top_right_midtone));
    top_painter.line_segment([r.right_bottom(),r.right_top()],egui::Stroke::new(stroke_width,bottom_right_shadow));
    top_painter.line_segment([r.right_bottom(),r.left_bottom()],egui::Stroke::new(stroke_width,bottom_right_shadow));
    top_painter.line_segment([r.left_bottom(),r.right_bottom()],egui::Stroke::new(stroke_width,bottom_left_midtone));
    top_painter.line_segment([r.left_bottom(),r.left_top()],egui::Stroke::new(stroke_width,bottom_left_midtone));
    top_painter.line_segment([r.left_top(),r.left_bottom()],egui::Stroke::new(stroke_width,top_left_highlight));
  }
  fn on_exit(&mut self) {
    println!("{}  {}:\n\n  {}\n",self.config.persona.emoji,self.config.persona.name,self.config.persona.dismissal);
  }
}
