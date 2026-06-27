use crossbeam_channel::{unbounded,Receiver};
use ratatui::{
  crossterm::event::{self, Event, KeyCode, KeyEventKind},
  crossterm::terminal,
  prelude::*,
  style::Style,
  widgets::*,
};
use std::time::Duration;

use crate::chat::*;
use crate::config::*;
use crate::defaults::*;
use crate::splash;

///////////// TUI

#[derive(Clone)]
#[allow(dead_code)]
pub(crate) struct TUI {
  pub(crate) config:        Config,
  pub(crate) chat:          Chat,
  pub(crate) recv:          Receiver<StreamEvent>,
  pub(crate) input:         String,
  pub(crate) history_lines: Vec<(String, String)>,
  pub(crate) current_reply: String,
  pub(crate) is_answering:  bool,
  pub(crate) scroll_offset: u16,
  pub(crate) user_scrolled: bool,
}

impl TUI {
  pub(crate) fn new(config: &Config) -> TUI {
    let (send, recv) = unbounded::<StreamEvent>();
    let initial_greeting = vec![(APP_NAME.to_string(), DEFAULT_GREETING.to_string())];
    TUI {
      config:        config.clone(),
      chat:          Chat::new(&config,send),
      recv:          recv,
      input:         String::new(),
      history_lines: initial_greeting,
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
          while let Ok(event) = self.recv.try_recv() {
            match event {
              StreamEvent::Token(token) => self.current_reply.push_str(&token),
              StreamEvent::Finished => {
                self.history_lines.push((format!("{}",APP_NAME), self.current_reply.clone()));
                self.current_reply.clear();
                self.is_answering = false;
              },
            }
          }
          let mut user_wants_to_exit = false;
          if let Err(err) = terminal.draw(|frame| {
            let full_area = frame.area();
            let raw_splash = splash::raw_splash().trim().to_string();
            let (splash_width, splash_height) = Self::get_text_dimensions(&raw_splash);
            let horizontal_chunks = Layout::default()
              .direction(Direction::Horizontal)
              .constraints([
                Constraint::Length(splash_width + 2),
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
                Constraint::Length(splash_height + 2),
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
                Constraint::Length(splash_height + 2),
                Constraint::Min(0),
              ])
              .split(left_column);
            let left_splash_pane = vertical_chunks[0];
            let splash: Text = Text::from(raw_splash).fg(Color::Rgb(91,55,36));
            let splash_paragraph = Paragraph::new(splash)
              .alignment(Alignment::Center)
              .block(Block::default()
              .title(ratatui::text::Line::from(format!(" {} ",APP_NAME)).fg(Color::Rgb(245,235,215)).bold())
              .title_alignment(Alignment::Center)
              .borders(Borders::ALL)
              .border_type(BorderType::Rounded)
              .border_style(Style::default().fg(Color::Rgb(194,162,105))));
            frame.render_widget(splash_paragraph,left_splash_pane);
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
                ratatui::text::Line::from(format!("{}  {}:",GROMRIK_EMOJI,sender)).fg(Color::Rgb(194, 162, 105)).bold()
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
              let active_header = ratatui::text::Line::from(format!("{}  {}:",GROMRIK_EMOJI,APP_NAME))
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
                format!(" {} is writing... ",APP_NAME)
            } else {
                format!(" Ask {} (Type 'exit' to quit) ",APP_NAME)
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
                  KeyCode::Up => {
                    self.user_scrolled = true;
                    if self.scroll_offset > 0 {
                      self.scroll_offset -= 1;
                    }
                  },
                  KeyCode::Down => {
                    if self.user_scrolled {
                      self.scroll_offset = self.scroll_offset.saturating_add(1);
                    }
                  },
                  KeyCode::Enter => {
                    if !self.input.is_empty() && !self.is_answering {
                      let prompt = self.input.trim().to_string();
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
                          "/exit" => user_wants_to_exit = true,
                          _ => {
                            self.history_lines.push(("System".to_string(),format!("Unknown command: '{}'. Type /clear to wipe the log.", command)));
                            self.user_scrolled = false;
                            self.input.clear();
                          },
                        }
                      } else if prompt.eq_ignore_ascii_case("exit") {
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
