#![cfg(feature = "tui")]

use crossbeam_channel::{unbounded,Receiver};
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  crossterm::terminal,
  prelude::*,
  style::Style,
  widgets::*,
};
use std::time::Duration;
use std::io;

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;

///////////// TUI

#[derive(Clone)]
pub(crate) struct TUI {
  pub(crate) config:        Config,
  pub(crate) chat:          Chat,
  pub(crate) recv:          Receiver<StreamEvent>,
  pub(crate) input:         String,
  pub(crate) history_lines: Vec<(String, String)>,
  pub(crate) input_history: Vec<String>,
  pub(crate) input_index:   usize,
  pub(crate) current_reply: String,
  pub(crate) is_answering:  bool,
  pub(crate) scroll_offset: u16,
  pub(crate) user_scrolled: bool,
}

impl TUI {
  pub(crate) fn new(config: &Config) -> TUI {
    let (send, recv) = unbounded::<StreamEvent>();
    let initial_greeting = vec![(config.persona.name.clone(),config.persona.greeting.clone())];
    TUI {
      config:        config.clone(),
      chat:          Chat::new(&config,send),
      recv:          recv,
      input:         String::new(),
      history_lines: initial_greeting,
      input_history: Vec::new(),
      input_index:   0,
      current_reply: String::new(),
      is_answering:  false,
      scroll_offset: 0,
      user_scrolled: false,
    }
  }

  pub(crate) fn run(&mut self) {
    match terminal::enable_raw_mode() {
      Ok(()) =>  {
        let mut terminal = ratatui::init();
        loop {
          let mut received_tokens = false;
          while let Ok(event) = self.recv.try_recv() {
            match event {
              StreamEvent::Token(token) => {
                self.current_reply.push_str(&token);
                received_tokens = true;
              },
              StreamEvent::Finished => {
                if !self.is_answering {
                  continue;
                }
                if !received_tokens && self.current_reply.trim().is_empty() {
                  log::error!("Failed to reach Ollama. Check your connection!");
                  self.history_lines.push((self.config.persona.name.clone(),self.config.persona.dismissal.clone()));
                } else {
                  log::debug!("Response from Ollama completed: \n\n{}\n",self.current_reply);
                  self.history_lines.push((self.config.persona.name.clone(), self.current_reply.clone()));
                }
                self.current_reply.clear();
                self.is_answering = false;
              },
            }
          }
          let mut user_wants_to_exit = false;
          if let Err(err) = terminal.draw(|frame| {
            let full_area = frame.area();
            let text_image = self.config.persona.text_image.clone();
            let (raw_persona_image_width,raw_persona_image_height) = Self::get_text_dimensions(&text_image);
            let persona_image_width  = if raw_persona_image_width < 60  { 60 } else { raw_persona_image_width };
            let persona_image_height = if raw_persona_image_height < 25 { 25 } else { raw_persona_image_height };
            let horizontal_chunks = Layout::default()
              .direction(Direction::Horizontal)
              .constraints([
                Constraint::Length(persona_image_width + 2),
                Constraint::Min(0),
              ])
              .split(full_area);
            let left_column = horizontal_chunks[0];
            let right_pane = horizontal_chunks[1];
            let chat_layout_chunks = Layout::default()
              .direction(Direction::Horizontal)
              .constraints([
                Constraint::Length(50),
                Constraint::Min(0),
              ])
              .split(right_pane);
            let chat_column = chat_layout_chunks[0];
            let chat_vertical_chunks = Layout::default()
              .direction(Direction::Vertical)
              .constraints([
                Constraint::Length(persona_image_height + 2),
                Constraint::Min(0),
              ])
              .split(chat_column);

            let chat_components = Layout::default()
              .direction(Direction::Vertical)
              .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
              ])
              .split(chat_vertical_chunks[0]);
            let chat_history_pane = chat_components[0];
            let chat_input_pane = chat_components[1];
            let vertical_chunks = Layout::default()
              .direction(Direction::Vertical)
              .constraints([
                Constraint::Length(persona_image_height + 2),
                Constraint::Min(0),
              ])
              .split(left_column);
            let left_persona_image_pane = vertical_chunks[0];
            let persona_image: Text = Text::from(text_image).fg(Color::Rgb(130,108,72));
            let persona_block = Block::default()
              .title(ratatui::text::Line::from(format!(" {} ", self.config.persona.name)).fg(Color::Rgb(245,235,215)).bold())
              .title_alignment(Alignment::Center)
              .borders(Borders::ALL)
              .border_type(BorderType::Rounded)
              .border_style(Style::default().fg(Color::Rgb(25,145,95)));
            let inner_height = left_persona_image_pane.height.saturating_sub(2);
            if (raw_persona_image_height as u16) < inner_height {
              frame.render_widget(persona_block, left_persona_image_pane);
              let inner_area = left_persona_image_pane.inner(ratatui::layout::Margin { horizontal: 1, vertical: 1 });
              let top_padding = (inner_height - raw_persona_image_height as u16) / 2;
              let inner_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                  Constraint::Length(top_padding),              // Exact blank spacer rows
                  Constraint::Length(raw_persona_image_height as u16),    // Lock art lines perfectly
                  Constraint::Min(0),
                ])
                .split(inner_area);
              let persona_image_paragraph = Paragraph::new(persona_image).alignment(Alignment::Center);
              frame.render_widget(persona_image_paragraph, inner_chunks[1]);
            } else {
              let persona_image_paragraph = Paragraph::new(persona_image)
                .alignment(Alignment::Center)
                .block(persona_block);
              frame.render_widget(persona_image_paragraph, left_persona_image_pane);
            }
            let history_block = Block::default()
              .title(ratatui::text::Line::from(" Conversation ").fg(Color::Rgb(165,235,200)).bold())
              .title_alignment(Alignment::Center)
              .borders(Borders::ALL)
              .border_type(BorderType::Rounded)
              .border_style(Style::default().fg(Color::Rgb(25,145,95)))
              .padding(ratatui::widgets::Padding::uniform(1));
            let inner_area = history_block.inner(chat_history_pane);
            let inner_width = inner_area.width as usize;
            let inner_height = inner_area.height;
            let mut text_spans = Vec::new();
            for (sender, content) in &self.history_lines {
              let header_line = if sender == "You" {
                ratatui::text::Line::from(format!("{} {}:",HUMAN_EMOJI,sender)).fg(Color::Rgb(100, 180, 220)).bold()
              } else if sender == "System" {
                ratatui::text::Line::from(format!("{}:", sender)).fg(Color::Rgb(215, 55, 55)).bold()
              } else {
                ratatui::text::Line::from(format!("{}  {}:",self.config.persona.emoji,sender)).fg(Color::Rgb(194, 162, 105)).bold()
              };
              text_spans.push(header_line);
              text_spans.push(ratatui::text::Line::from("")); 
              let wrapped_lines = Self::wrap_and_indent_text(content, inner_width);
              for mut line in wrapped_lines {
                line = line.fg(Color::Rgb(240, 240, 245)).not_dim(); // Pure bright off-white text [local]
                text_spans.push(line);
              }
              text_spans.push(ratatui::text::Line::from(""));
            }
            if self.is_answering && !self.current_reply.is_empty() {
              let active_header = ratatui::text::Line::from(format!("{}  {}:",self.config.persona.emoji,self.config.persona.name))
                .fg(Color::Rgb(194, 162, 105))
                .bold();
              text_spans.push(active_header);
              text_spans.push(ratatui::text::Line::from("")); 
              let wrapped_reply_lines = Self::wrap_and_indent_text(&self.current_reply, inner_width);
              for mut line in wrapped_reply_lines {
                line = line.fg(Color::Rgb(240, 240, 245)).not_dim(); 
                text_spans.push(line);
              }
            }
            let total_lines = text_spans.len() as u16;
            if total_lines > inner_height {
              let max_scroll = total_lines - inner_height;
              if !self.user_scrolled {
                self.scroll_offset = max_scroll;
              } else {
                if self.scroll_offset > max_scroll {
                  self.scroll_offset = max_scroll;
                }
              }
            } else {
              self.scroll_offset = 0;
              self.user_scrolled = false;
            }
            let history_text = Paragraph::new(text_spans)
              .wrap(Wrap { trim: false })
              .scroll((self.scroll_offset, 0))
              .block(history_block);
            frame.render_widget(history_text, chat_history_pane);
            let input_title = if self.is_answering {
                format!(" {} is writing... ",self.config.persona.name)
            } else {
                format!(" Ask {} (Type 'exit' to quit) ",self.config.persona.name)
            };
            let input_border_color = if self.is_answering {
              Color::Rgb(115, 120, 125)
            } else {
              Color::Rgb(255, 190, 105)
            };
            let input_block = Block::default()
              .title(ratatui::text::Line::from(input_title).fg(Color::Rgb(255,230,235)).bold())
              .borders(Borders::ALL)
              .border_type(BorderType::Rounded)
              .border_style(Style::default().fg(input_border_color));
            let input_text = Paragraph::new(format!("> {}", self.input))
              .fg(Color::Rgb(255, 255, 255))
              .not_dim() 
              .block(input_block);
            frame.render_widget(input_text, chat_input_pane);
          }) { log::error!("Failed to display split interface: {}", err); }
          if event::poll(Duration::from_millis(15)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
              if key.kind == KeyEventKind::Press {
                match key.code {
                  KeyCode::Tab => {
                    if !self.is_answering {
                      let commands = vec![
                        "/clear", "/reset", "/persona ", "/save ", 
                        "/load ", "/exit", "/quit", "/help"
                      ];
                      
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
                          }
                        }
                        else if current_input.starts_with("/load ") {
                          let argument = &self.input["/load ".len()..];
                          if argument.trim().is_empty() {
                            self.input = format!("/load {}", self.config.history_file);
                          }
                        }
                        else if current_input.starts_with("/save ") {
                          let argument = &self.input["/save ".len()..];
                          if argument.trim().is_empty() {
                            self.input = format!("/save {}", self.config.history_file);
                          }
                        }
                        else if self.input.chars().count() > 1 {
                          let is_already_exact_command = commands.iter().any(|cmd| current_input.starts_with(*cmd));
                          if !is_already_exact_command {
                            if let Some(matched_command) = commands.iter().find(|cmd| cmd.starts_with(&current_input)) {
                              self.input = matched_command.to_string();
                            }
                          }
                        }
                      }
                    }
                  },
                  KeyCode::Char(c) => {
                    if !self.is_answering {
                      self.input.push(c);
                    }
                  },
                  KeyCode::Backspace => {
                    if !self.is_answering {
                      self.input.pop();
                    }
                  },
                  KeyCode::PageUp => {
                    self.user_scrolled = true;
                    if self.scroll_offset > 0 {
                      self.scroll_offset -= 1;
                    }
                  },
                  KeyCode::PageDown => {
                    if self.user_scrolled {
                      self.scroll_offset = self.scroll_offset.saturating_add(1);
                    }
                  },
                  KeyCode::Up => {
                    if !self.input_history.is_empty() && self.input_index > 0 {
                      self.input_index -= 1;
                      self.input = self.input_history[self.input_index].clone();
                    }
                  },
                  KeyCode::Down => {
                    if !self.input_history.is_empty() {
                      self.input_index = self.input_index.saturating_add(1);
                      if self.input_index < self.input_history.len() {
                        self.input = self.input_history[self.input_index].clone();
                      } else {
                        self.input_index = self.input_history.len();
                        self.input.clear();
                      }
                    }
                  },
                  KeyCode::Enter => {
                    if !self.input.is_empty() && !self.is_answering {
                      let prompt = self.input.trim().to_string();
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
                            self.scroll_offset = 0;
                            self.user_scrolled = false;
                            self.input.clear();
                          },
                          "/reset" => {
                            let _ = terminal.clear();
                            print!("\x1B[2J\x1B[1;1H"); 
                            let _ = io::stdout().flush();
                            *self = Self::new(&self.config.clone());
                          }
                          "/persona" => {
                            if parts.len() > 1 {
                              let label = parts[1..].join(" ");
                              if let Some (persona) = self.config.personas.get(&label) {
                                let mut config: Config = self.config.clone();
                                config.persona = persona.clone();
                                *self = Self::new(&config);
                                let _ = terminal.clear();
                                print!("\x1B[2J\x1B[1;1H"); 
                                let _ = io::stdout().flush();
                              } else {
                                self.history_lines.push(("System".to_string(),format!("Please provide a valid persona: {}", self.config.personas.keys().cloned().collect::<Vec<String>>().join(","))));
                              }
                            } else {
                              self.history_lines.push(("System".to_string(),format!("Please provide a valid persona: {}", self.config.personas.keys().cloned().collect::<Vec<String>>().join(","))));
                            }
                          },
                          "/save" => {
                            let filename: String = if parts.len() > 1 { parts[1..].join(" ") } else { self.config.history_file.clone() };
                            let msg = match self.chat.save_history(&filename.to_string(), &self.history_lines) {
                              Ok(_) => format!("History successfully saved to {}.", filename),
                              Err(err) => format!("Save error: {}",err)
                            };
                            self.history_lines.push(("System".to_string(),msg));
                            self.scroll_offset = 0;
                            self.user_scrolled = false;
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
                            self.scroll_offset = 0;
                            self.user_scrolled = false;
                            self.input.clear();
                          },
                          "/exit"|"/quit" => user_wants_to_exit = true,
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
                            self.scroll_offset = 0;
                            self.user_scrolled = false;
                            self.input.clear();
                          },
                          _ => {
                            self.history_lines.push(("System".to_string(),format!("Unknown command: '{}'. Type /help for available commands.", command)));
                            self.user_scrolled = false;
                            self.input.clear();
                          },
                        }
                      } else if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
                        user_wants_to_exit = true;
                      } else {
                        self.user_scrolled = false; 
                        self.scroll_offset = 0;
                        self.history_lines.push(("You".to_string(), prompt.clone()));
                        self.is_answering = true;
                        if let Err(e) = self.chat.chat(&prompt) {
                          log::error!("Failed to launch chat context thread: {}", e);
                          self.is_answering = false;
                        }
                        self.input.clear(); 
                      }
                    }
                  },
                  _ => {},
                }
              }
            }
          }
          if user_wants_to_exit {
            break;
          }
        }
        ratatui::restore();
        println!("{}  {}:\n\n  {}\n",self.config.persona.emoji,self.config.persona.name,self.config.persona.dismissal);
      },
      Err(_) => {},
    }
  }

  fn get_text_dimensions(text: &str) -> (u16, u16) {
    let lines: Vec<&str> = text.lines().collect();
    let height = lines.len() as u16;
    let width = lines.iter()
      .map(|line| line.chars().count())
      .max()
      .unwrap_or(0) as u16;
    (width, height)
  }

  fn wrap_and_indent_text(text: &str, max_width: usize) -> Vec<ratatui::text::Line<'static>> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
      let trimmed = paragraph.trim();
      if trimmed.is_empty() {
        lines.push(ratatui::text::Line::from(""));
        continue;
      }
      let mut current_line = String::from("  ");
      for word in paragraph.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
          lines.push(ratatui::text::Line::from(current_line.clone()));
          current_line = String::from("  ");
        }
        if current_line.len() > 2 {
          current_line.push(' ');
        }
        current_line.push_str(word);
      }
      if current_line.len() > 2 {
        lines.push(ratatui::text::Line::from(current_line));
      }
    }
    lines
  }
}
