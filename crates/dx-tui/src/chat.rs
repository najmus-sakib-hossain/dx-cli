use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

use dx_ai::{GeminiClient, Message as GeminiMessage};
use dx_core::config::DxConfig;

pub struct ChatApp {
    input: String,
    messages: Vec<ChatMessage>,
    gemini: GeminiClient,
    gemini_history: Vec<GeminiMessage>,
    is_loading: bool,
    scroll_offset: usize,
}

#[derive(Clone)]
struct ChatMessage {
    role: MessageRole,
    content: String,
}

#[derive(Clone, PartialEq)]
enum MessageRole {
    User,
    Assistant,
    System,
    Command,
}

impl ChatApp {
    pub fn new(config: Option<DxConfig>) -> Self {
        let api_key = config.and_then(|c| c.ai.gemini_api_key);
        
        Self {
            input: String::new(),
            messages: vec![ChatMessage {
                role: MessageRole::System,
                content: "Dx AI Assistant - Type commands or ask questions. Press Ctrl+C to exit.".to_string(),
            }],
            gemini: GeminiClient::new(api_key),
            gemini_history: Vec::new(),
            is_loading: false,
            scroll_offset: 0,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal).await;

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        result
    }

    async fn run_app<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(());
                        }
                        KeyCode::Enter => {
                            if !self.input.is_empty() && !self.is_loading {
                                let input = self.input.clone();
                                self.input.clear();
                                self.handle_input(input).await?;
                            }
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Up => {
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if self.scroll_offset < self.messages.len().saturating_sub(10) {
                                self.scroll_offset += 1;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    async fn handle_input(&mut self, input: String) -> Result<()> {
        // Add user message
        self.messages.push(ChatMessage {
            role: MessageRole::User,
            content: input.clone(),
        });

        // Check if it's a command
        if self.is_command(&input) {
            self.execute_command(&input).await?;
        } else {
            // Send to AI
            self.is_loading = true;
            self.messages.push(ChatMessage {
                role: MessageRole::Assistant,
                content: "Thinking...".to_string(),
            });

            match self.gemini.chat(&input, &self.gemini_history).await {
                Ok(response) => {
                    // Remove "Thinking..." message
                    self.messages.pop();
                    
                    // Add AI response
                    self.messages.push(ChatMessage {
                        role: MessageRole::Assistant,
                        content: response.clone(),
                    });

                    // Update history
                    self.gemini_history.push(GeminiMessage::user(input));
                    self.gemini_history.push(GeminiMessage::model(response));
                }
                Err(e) => {
                    self.messages.pop();
                    self.messages.push(ChatMessage {
                        role: MessageRole::System,
                        content: format!("Error: {}", e),
                    });
                }
            }
            self.is_loading = false;
        }

        // Auto-scroll to bottom
        self.scroll_offset = self.messages.len().saturating_sub(10);

        Ok(())
    }

    fn is_command(&self, input: &str) -> bool {
        let trimmed = input.trim();
        
        // Check for shell commands
        let shell_commands = [
            "ls", "cd", "pwd", "cat", "echo", "mkdir", "rm", "cp", "mv", "grep",
            "cargo", "npm", "yarn", "git", "python", "node", "rustc", "code",
        ];
        
        // Check for dx commands
        let dx_commands = [
            "dx ui", "dx style", "dx icon", "dx font", "dx auth",
            "dx media", "dx i18n", "dx forge", "dx generate", "dx shell",
        ];

        shell_commands.iter().any(|cmd| trimmed.starts_with(cmd))
            || dx_commands.iter().any(|cmd| trimmed.starts_with(cmd))
            || trimmed.starts_with("./")
    }

    async fn execute_command(&mut self, command: &str) -> Result<()> {
        self.messages.push(ChatMessage {
            role: MessageRole::Command,
            content: format!("Executing: {}", command),
        });

        // Execute command using tokio
        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .await;

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                let mut result = String::new();
                if !stdout.is_empty() {
                    result.push_str(&stdout);
                }
                if !stderr.is_empty() {
                    if !result.is_empty() {
                        result.push('\n');
                    }
                    result.push_str(&stderr);
                }
                
                self.messages.push(ChatMessage {
                    role: MessageRole::Command,
                    content: if result.is_empty() {
                        "Command executed successfully (no output)".to_string()
                    } else {
                        result
                    },
                });
            }
            Err(e) => {
                self.messages.push(ChatMessage {
                    role: MessageRole::System,
                    content: format!("Failed to execute command: {}", e),
                });
            }
        }

        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),      // Messages area
                Constraint::Length(3),   // Input box
            ])
            .split(f.area());

        // Render messages
        self.render_messages(f, chunks[0]);

        // Render input box
        self.render_input(f, chunks[1]);
    }

    fn render_messages(&self, f: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .skip(self.scroll_offset)
            .map(|msg| {
                let (prefix, style) = match msg.role {
                    MessageRole::User => (
                        "▶ You",
                        Style::default().fg(Color::Rgb(0, 112, 243)), // Vercel blue
                    ),
                    MessageRole::Assistant => (
                        "● Assistant",
                        Style::default().fg(Color::Rgb(0, 200, 0)), // Green
                    ),
                    MessageRole::System => (
                        "• System",
                        Style::default().fg(Color::Rgb(160, 160, 160)), // Gray
                    ),
                    MessageRole::Command => (
                        "$ Command",
                        Style::default().fg(Color::Rgb(255, 200, 0)), // Yellow
                    ),
                };

                let content = vec![
                    Line::from(vec![
                        Span::styled(prefix, style.add_modifier(Modifier::BOLD)),
                    ]),
                    Line::from(msg.content.as_str()),
                    Line::from(""),
                ];

                ListItem::new(content)
            })
            .collect();

        let list = List::new(messages)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Plain)
                    .title(" Dx AI Chat ")
                    .style(Style::default().fg(Color::Rgb(48, 48, 48))),
            );

        f.render_widget(list, area);
    }

    fn render_input(&self, f: &mut Frame, area: Rect) {
        let input_text = if self.is_loading {
            format!("{} ⏳ Processing...", self.input)
        } else {
            self.input.clone()
        };

        let input = Paragraph::new(input_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(Color::Rgb(0, 112, 243))) // Vercel blue
                    .title(" Type your message or command... (Ctrl+C to exit) "),
            );

        f.render_widget(input, area);
    }
}
