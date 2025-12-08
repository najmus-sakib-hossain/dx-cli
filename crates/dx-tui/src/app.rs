use anyhow::Result;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Terminal;
use std::collections::VecDeque;
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

const TICK_RATE: Duration = Duration::from_millis(200);
const MAX_LOG_LINES: usize = 120;

#[derive(Clone, Debug)]
struct CommandItem {
    name: &'static str,
    description: &'static str,
    hint: &'static str,
}

#[derive(Debug)]
struct AppState {
    commands: Vec<CommandItem>,
    selected: usize,
    input: String,
    logs: VecDeque<String>,
    status: String,
}

#[derive(Clone, Debug)]
struct RenderState {
    commands: Vec<CommandItem>,
    selected: usize,
    input: String,
    logs: Vec<String>,
    status: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            commands: default_commands(),
            selected: 0,
            input: String::new(),
            logs: VecDeque::with_capacity(MAX_LOG_LINES),
            status: "Type a command or press Enter to run the highlighted one".to_string(),
        }
    }

    fn log(&mut self, message: impl Into<String>) {
        if self.logs.len() >= MAX_LOG_LINES {
            self.logs.pop_front();
        }
        self.logs.push_back(message.into());
    }

    fn snapshot(&self) -> RenderState {
        RenderState {
            commands: self.commands.clone(),
            selected: self.selected,
            input: self.input.clone(),
            logs: self.logs.iter().cloned().collect(),
            status: self.status.clone(),
        }
    }
}

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    state: AppState,
}

impl TuiApp {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            terminal,
            state: AppState::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut last_tick = Instant::now();

        loop {
            self.draw()?;

            let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && self.handle_key(key)? {
                        break;
                    }
                }
            }

            if last_tick.elapsed() >= TICK_RATE {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<bool> {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
            KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
            KeyCode::Up => {
                if self.state.selected > 0 {
                    self.state.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.state.selected + 1 < self.state.commands.len() {
                    self.state.selected += 1;
                }
            }
            KeyCode::Tab => {
                if let Some(item) = self.state.commands.get(self.state.selected) {
                    self.state.input = item.name.to_string();
                    self.state.status = format!("Staged '{}'; press Enter to run", item.name);
                }
            }
            KeyCode::Backspace => {
                self.state.input.pop();
            }
            KeyCode::Delete => {
                self.state.input.clear();
                self.state.status = "Input cleared".into();
            }
            KeyCode::Enter => {
                self.execute_current();
            }
            KeyCode::Char('?') => {
                self.state.status = "Shortcuts: ↑/↓ navigate · Tab stage command · Enter run · Ctrl+C/q exit".into();
            }
            KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.state.logs.clear();
                self.state.status = "Cleared activity log".into();
            }
            KeyCode::Char(ch) => {
                self.state.input.push(ch);
            }
            _ => {}
        }

        Ok(false)
    }

    fn execute_current(&mut self) {
        let mut cmd = self.state.input.trim().to_string();
        if cmd.is_empty() {
            if let Some(item) = self.state.commands.get(self.state.selected) {
                cmd = item.name.to_string();
            }
        }

        if cmd.is_empty() {
            self.state.status = "Nothing to run".into();
            return;
        }

        self.state.log(format!("→ {cmd}"));
        self.state.status = format!("Running '{cmd}'");

        let response = synthetic_execute(&cmd);

        if cmd.trim().eq_ignore_ascii_case("clear") {
            self.state.logs.clear();
        }
        for line in response {
            self.state.log(line);
        }

        self.state.input.clear();
        self.state.status = "Ready".into();
    }

    fn draw(&mut self) -> Result<()> {
        let render_state = self.state.snapshot();

        self.terminal.draw(|frame| {
            let size = frame.area().inner(Margin {
                vertical: 0,
                horizontal: 0,
            });

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(5),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ])
                .split(size);

            render_header(frame, layout[0]);
            render_body(frame, layout[1], &render_state);
            render_footer(frame, layout[2], &render_state);
        })?;

        Ok(())
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
    }
}

fn render_header(frame: &mut ratatui::Frame<'_>, area: Rect) {
    let title = Line::from(vec![
        Span::styled("DX", Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(" /", Style::default().fg(Color::Rgb(64, 64, 64))),
        Span::styled(" Enhanced Development Experience", Style::default().fg(Color::Rgb(160, 160, 160))),
    ]);

    let subtitle = Line::from(vec![
        Span::styled("Fast", Style::default().fg(Color::Rgb(0, 112, 243))),
        Span::styled(" • ", Style::default().fg(Color::Rgb(64, 64, 64))),
        Span::styled("AI-First", Style::default().fg(Color::Rgb(0, 112, 243))),
        Span::styled(" • ", Style::default().fg(Color::Rgb(64, 64, 64))),
        Span::styled("Transparent", Style::default().fg(Color::Rgb(0, 112, 243))),
    ]);

    let header = Paragraph::new(vec![Line::raw(""), title, subtitle])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(48, 48, 48))),
        )
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(header, area);
}

fn render_body(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RenderState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
        .split(area);

    render_palette(frame, chunks[0], state);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(chunks[1]);

    render_logs(frame, right[0], state);
    render_input(frame, right[1], state);
}

fn render_palette(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RenderState) {
    let items: Vec<ListItem> = state
        .commands
        .iter()
        .map(|cmd| {
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(cmd.name, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(cmd.description, Style::default().fg(Color::Rgb(128, 128, 128))),
                ]),
                Line::raw(""),
            ])
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(48, 48, 48)))
                .title(Span::styled(" Commands ", Style::default().fg(Color::Rgb(160, 160, 160))))
                .title_alignment(ratatui::layout::Alignment::Left),
        )
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(0, 112, 243))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ")
        .repeat_highlight_symbol(true)
        .highlight_spacing(HighlightSpacing::Always);

    let mut list_state = {
        let mut s = ListState::default();
        s.select(Some(state.selected));
        s
    };
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_logs(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RenderState) {
    let lines: Vec<Line> = state
        .logs
        .iter()
        .rev()
        .take(50)
        .rev()
        .map(|entry| {
            let styled = if entry.starts_with('→') {
                Line::from(vec![
                    Span::styled("  → ", Style::default().fg(Color::Rgb(0, 112, 243))),
                    Span::raw(entry.trim_start_matches('→').trim()),
                ])
            } else if entry.contains('✓') {
                Line::from(vec![
                    Span::styled("  ✓ ", Style::default().fg(Color::Rgb(0, 200, 0))),
                    Span::raw(entry.trim_start_matches('✓').trim()),
                ])
            } else {
                Line::from(vec![
                    Span::styled("    ", Style::default()),
                    Span::raw(entry.clone()),
                ])
            };
            styled
        })
        .collect();

    let paragraph = Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(Span::styled(" Activity ", Style::default().fg(Color::Rgb(160, 160, 160))))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(48, 48, 48))),
        );

    frame.render_widget(paragraph, area);
}

fn render_input(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RenderState) {
    let cursor = Span::styled("│", Style::default().fg(Color::Rgb(0, 112, 243)));
    let content = Line::from(vec![
        Span::styled(" $ ", Style::default().fg(Color::Rgb(0, 112, 243)).add_modifier(Modifier::BOLD)),
        Span::styled(&state.input, Style::default().fg(Color::White)),
        cursor,
    ]);

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .title(Span::styled(" Input ", Style::default().fg(Color::Rgb(160, 160, 160))))
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(48, 48, 48))),
        );

    frame.render_widget(paragraph, area);
}

fn render_footer(frame: &mut ratatui::Frame<'_>, area: Rect, state: &RenderState) {
    let hints = Line::from(vec![
        Span::styled("[↑↓]", Style::default().fg(Color::Rgb(128, 128, 128))),
        Span::raw(" Navigate  "),
        Span::styled("[Tab]", Style::default().fg(Color::Rgb(128, 128, 128))),
        Span::raw(" Stage  "),
        Span::styled("[Enter]", Style::default().fg(Color::Rgb(128, 128, 128))),
        Span::raw(" Execute  "),
        Span::styled("[q]", Style::default().fg(Color::Rgb(128, 128, 128))),
        Span::raw(" Quit"),
    ]);

    let status_line = Line::from(vec![
        Span::styled("● ", Style::default().fg(Color::Rgb(0, 200, 0))),
        Span::styled(&state.status, Style::default().fg(Color::Rgb(200, 200, 200))),
    ]);

    let status = Paragraph::new(vec![status_line, hints])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain)
                .border_style(Style::default().fg(Color::Rgb(48, 48, 48))),
        );

    frame.render_widget(status, area);
}

fn default_commands() -> Vec<CommandItem> {
    vec![
        CommandItem {
            name: "ui list",
            description: "Browse available UI components",
            hint: "dx ui --list",
        },
        CommandItem {
            name: "style compile",
            description: "Compile styles with optimizer",
            hint: "dx style --name base.css",
        },
        CommandItem {
            name: "icon fetch",
            description: "Sync icon sets (lucide, heroicons)",
            hint: "dx icon --list",
        },
        CommandItem {
            name: "font install",
            description: "Install curated font packs",
            hint: "dx font --name inter",
        },
        CommandItem {
            name: "auth configure",
            description: "Set up auth providers",
            hint: "dx auth --name github",
        },
        CommandItem {
            name: "media optimize",
            description: "Optimize media assets",
            hint: "dx media --name assets/hero.png",
        },
        CommandItem {
            name: "i18n generate",
            description: "Generate localization bundles",
            hint: "dx i18n --list",
        },
        CommandItem {
            name: "forge status",
            description: "Check VCS pipeline state",
            hint: "dx forge --list",
        },
        CommandItem {
            name: "generate project",
            description: "Start a new project from template",
            hint: "dx generate --kind project",
        },
        CommandItem {
            name: "generate component button",
            description: "Scaffold a UI component",
            hint: "dx generate --kind component --template button",
        },
        CommandItem {
            name: "chat hello",
            description: "Ping Friday AI",
            hint: "dx chat --message 'hello'",
        },
        CommandItem {
            name: "agent run",
            description: "Execute an agent task",
            hint: "dx agent --task 'analyze repo'",
        },
        CommandItem {
            name: "shell status",
            description: "Show shell enhancement status",
            hint: "dx shell",
        },
    ]
}

fn synthetic_execute(command: &str) -> Vec<String> {
    let trimmed = command.trim().to_lowercase();

    let mut lines = Vec::new();
    if trimmed.starts_with("ui") {
        lines.push("✓UI toolkit ready — components: button, accordion, modal".to_string());
    } else if trimmed.starts_with("style") {
        lines.push("✓Styles compiled with minify=true".to_string());
    } else if trimmed.starts_with("icon") {
        lines.push("✓Icon sets synced (lucide, heroicons)".to_string());
    } else if trimmed.starts_with("font") {
        lines.push("✓Fonts installed to public/fonts".to_string());
    } else if trimmed.starts_with("auth") {
        lines.push("✓Auth provider configured; secrets stored securely".to_string());
    } else if trimmed.starts_with("media") {
        lines.push("✓Media optimized; saved 32% disk".to_string());
    } else if trimmed.starts_with("i18n") {
        lines.push("✓Localization bundles generated for en/es/fr".to_string());
    } else if trimmed.starts_with("forge") {
        lines.push("✓Forge pipeline healthy; branch dx/main clean".to_string());
    } else if trimmed.starts_with("generate component") {
        lines.push("✓Component scaffolded (dry-run)".to_string());
    } else if trimmed.starts_with("generate") {
        lines.push("✓Project template rendered to ./out".to_string());
    } else if trimmed.starts_with("chat") {
        lines.push("✓Friday: Hey there! Ready to build something great.".to_string());
    } else if trimmed.starts_with("agent") {
        lines.push("✓Agent orchestrator kicked off".to_string());
    } else if trimmed.starts_with("shell") {
        lines.push("✓Shell enhancements active (fuzzy, history, ai)".to_string());
    } else if trimmed == "clear" {
        lines.push("(log cleared)".to_string());
    } else {
        lines.push(format!("Executed '{command}' (stub)"));
    }

    lines
}
