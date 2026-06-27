use crossbeam_channel::{unbounded,Receiver};
use eframe::egui;

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// GUI

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct GUI {
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
    let initial_greeting = vec![(APP_NAME.to_string(),DEFAULT_GREETING.to_string())];
    GUI {
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
        .with_inner_size([800.0, 600.0]) 
        .with_resizable(false),          
      ..Default::default()
    };
    let instance = self;
    let _ = eframe::run_native(
      "Gromrik",
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
    while let Ok(event) = self.recv.try_recv() {
      match event {
        StreamEvent::Token(token) => {
          self.current_reply.push_str(&token);
          self.scroll_to_bottom = true;
        }
        StreamEvent::Finished => {
          self.history_lines.push((APP_NAME.to_string(), self.current_reply.clone()));
          self.current_reply.clear();
          self.is_answering = false;
          self.scroll_to_bottom = true;
        }
      }
    }
    let image_source = egui::ImageSource::Bytes {
      uri: std::borrow::Cow::Borrowed("bytes://gromrik_bg.png"),
      bytes: egui::load::Bytes::from(GROMRIK_FULL_IMAGE),
    };
    ui.add(
      egui::Image::new(image_source).max_size(egui::vec2(ui.available_width(), ui.available_height()))
    );
    let left_pixel_x   = 555.0;
    let top_pixel_y    = 35.0;
    let panel_width    = 215.0;
    let panel_height   = 500.0;
    let target_rect = egui::Rect::from_min_size(
      egui::pos2(left_pixel_x, top_pixel_y),
      egui::vec2(panel_width, panel_height)
    );
    let ui_builder = egui::UiBuilder::new()
      .max_rect(target_rect)
      .layout(egui::Layout::top_down(egui::Align::LEFT));
    ui.scope_builder(ui_builder, |ui| {
      egui::ScrollArea::vertical()
        .max_height(panel_height)
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
              ui.add(egui::Label::new(egui::RichText::new(format!("{}:", APP_NAME)).color(egui::Color32::from_rgb(194, 162, 105)).font(header_font)));
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
    let input_left_x   = 560.0; 
    let input_top_y    = 550.0;
    let input_width    = 210.0; 
    let input_height   = 30.0;
    let input_rect = egui::Rect::from_min_size(
      egui::pos2(input_left_x, input_top_y),
      egui::vec2(input_width, input_height)
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
        .desired_width(input_width)
        .text_color(egui::Color32::from_rgb(245,235,215)) 
        .hint_text("Ask Gromrik...")
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
              "/exit" => ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close),
              _ => {
                self.history_lines.push(("System".to_string(),format!("Unknown command: '{}'. Type /clear to wipe the log.", command)));
                self.scroll_to_bottom = true;
                self.input.clear();
              },
            }
          } else if prompt.eq_ignore_ascii_case("exit") {
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
}
