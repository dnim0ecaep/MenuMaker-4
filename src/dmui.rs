//! Shared DeskMate-style drawing: theme-colored double-line boxes with a
//! centered `◄ TITLE ►` band, the app's header/shortcut/status bars, and a
//! small labelled-field dialog used by the Address Book and Calendar.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Clear, Paragraph};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

/// The theme colors the box drawing needs.
#[derive(Clone, Copy)]
pub struct Palette {
    pub primary: Color,
    pub accent: Color,
    pub highlight: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub bar_bg: Color,
    pub bar_key: Color,
    pub bar_label: Color,
    pub selection_text: Color,
    /// Focused title band background, when the theme sets one.
    pub focus_band: Option<Color>,
}

impl Palette {
    pub fn fill(&self) -> Style {
        Style::default().bg(self.surface).fg(self.text)
    }

    pub fn selected(&self) -> Style {
        Style::default()
            .fg(self.selection_text)
            .bg(self.highlight)
            .add_modifier(Modifier::BOLD)
    }

    fn focused_band(&self) -> Style {
        match self.focus_band {
            Some(band) => Style::default()
                .fg(self.accent)
                .bg(band)
                .add_modifier(Modifier::BOLD),
            None => self.selected(),
        }
    }

    pub fn label(&self) -> Style {
        Style::default()
            .fg(self.accent)
            .bg(self.surface)
            .add_modifier(Modifier::BOLD)
    }
}

/// Truncate `text` to at most `width` display columns.
pub fn fit_width(text: &str, width: usize) -> String {
    let mut out = String::new();
    let mut used = 0;
    for ch in text.chars() {
        let w = UnicodeWidthStr::width(ch.to_string().as_str());
        if used + w > width {
            break;
        }
        used += w;
        out.push(ch);
    }
    out
}

/// Pad (or cut) `text` to exactly `width` display columns.
pub fn pad_width(text: &str, width: usize) -> String {
    let fitted = fit_width(text, width);
    let used = UnicodeWidthStr::width(fitted.as_str());
    format!("{fitted}{}", " ".repeat(width.saturating_sub(used)))
}

/// Draw a DeskMate box: double-line frame in the accent color, filled with
/// `fill`, and the title centered on the top border as `◄ TITLE ►`. The band
/// uses the selection colors when `focused`. Returns the inner area.
pub fn dm_box(frame: &mut Frame, area: Rect, title: &str, pal: &Palette, fill: Style, focused: bool) -> Rect {
    if area.width < 2 || area.height < 2 {
        return Rect::default();
    }
    let fill_bg = fill.bg.unwrap_or(pal.surface);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(pal.accent).bg(fill_bg))
            .style(fill),
        area,
    );
    if area.width > 6 && !title.is_empty() {
        let band = fit_width(&format!(" ◄ {title} ► "), area.width as usize - 4);
        let band_width = UnicodeWidthStr::width(band.as_str()) as u16;
        let band_style = if focused {
            pal.focused_band()
        } else {
            Style::default()
                .fg(fill_bg)
                .bg(pal.accent)
                .add_modifier(Modifier::BOLD)
        };
        frame.render_widget(
            Paragraph::new(band).style(band_style),
            Rect {
                x: area.x + (area.width - band_width) / 2,
                y: area.y,
                width: band_width,
                height: 1,
            },
        );
    }
    area.inner(&Margin {
        horizontal: 1,
        vertical: 1,
    })
}

/// Draw one line of text into row `row` of `area`, padded to its width.
pub fn put_line(frame: &mut Frame, area: Rect, row: u16, text: &str, style: Style) {
    if row >= area.height || area.width == 0 {
        return;
    }
    frame.render_widget(
        Paragraph::new(pad_width(text, area.width as usize)).style(style),
        Rect {
            x: area.x,
            y: area.y + row,
            width: area.width,
            height: 1,
        },
    );
}

/// Draw the app chrome used by every full screen: title bar, shortcut bar,
/// and status bar. Returns the desktop area between them, painted with the
/// theme background.
pub fn screen_chrome(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    shortcuts: &[(&str, &str)],
    status: &str,
    pal: &Palette,
) -> Rect {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);
    let bar = Style::default()
        .bg(pal.primary)
        .fg(pal.text)
        .add_modifier(Modifier::BOLD);
    frame.render_widget(Clear, area);
    frame.render_widget(Paragraph::new(title.to_string()).alignment(Alignment::Center).style(bar), chunks[0]);

    let key_style = Style::default()
        .fg(pal.bar_key)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(pal.bar_label);
    let mut spans = Vec::new();
    for (idx, (key, label)) in shortcuts.iter().enumerate() {
        if idx > 0 {
            spans.push(Span::styled(" | ", label_style));
        }
        spans.push(Span::styled(key.to_string(), key_style));
        spans.push(Span::styled(format!(" {label}"), label_style));
    }
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .style(Style::default().bg(pal.bar_bg)),
        chunks[1],
    );
    frame.render_widget(Block::default().style(Style::default().bg(pal.background)), chunks[2]);
    frame.render_widget(Paragraph::new(status.to_string()).alignment(Alignment::Center).style(bar), chunks[3]);
    chunks[2].inner(&Margin {
        horizontal: 1,
        vertical: 1,
    })
}

/// Where [`screen_chrome`] draws each shortcut chip on its centered
/// shortcut bar, as `(x_offset, width)` relative to the bar's left edge, so
/// callers can make the chips clickable. Returns `None` when the chips
/// don't all fit in `width` (the bar would then be cut off).
pub fn shortcut_spans(width: u16, shortcuts: &[(&str, &str)]) -> Option<Vec<(u16, u16)>> {
    let mut spans = Vec::with_capacity(shortcuts.len());
    let mut x = 0usize;
    for (idx, (key, label)) in shortcuts.iter().enumerate() {
        if idx > 0 {
            x += 3; // " | "
        }
        let w = UnicodeWidthStr::width(*key) + 1 + UnicodeWidthStr::width(*label);
        spans.push((x, w));
        x += w;
    }
    if x > width as usize {
        return None;
    }
    // Same centering as `Paragraph` with `Alignment::Center`.
    let offset = (width / 2).saturating_sub(x as u16 / 2);
    Some(spans.into_iter().map(|(sx, w)| (offset + sx as u16, w as u16)).collect())
}

/// One field of a [`FieldForm`].
pub struct FormField {
    pub label: String,
    pub value: String,
    /// When set, the field picks one of these with ←/→/Space instead of typing.
    pub choices: Vec<String>,
}

impl FormField {
    pub fn text(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
            choices: Vec::new(),
        }
    }

    pub fn choice(label: &str, value: &str, choices: &[&str]) -> Self {
        let choices: Vec<String> = choices.iter().map(|c| c.to_string()).collect();
        let value = choices
            .iter()
            .find(|c| c.eq_ignore_ascii_case(value))
            .cloned()
            .unwrap_or_else(|| choices.first().cloned().unwrap_or_default());
        Self {
            label: label.to_string(),
            value,
            choices,
        }
    }

    fn cycle(&mut self, forward: bool) {
        if self.choices.is_empty() {
            return;
        }
        let len = self.choices.len();
        let pos = self.choices.iter().position(|c| *c == self.value).unwrap_or(0);
        let next = if forward { (pos + 1) % len } else { (pos + len - 1) % len };
        self.value = self.choices[next].clone();
    }
}

pub enum FormResult {
    Continue,
    Submit,
    Cancel,
}

/// A centered dialog of labelled single-line fields.
pub struct FieldForm {
    pub title: String,
    pub fields: Vec<FormField>,
    pub selected: usize,
    pub error: Option<String>,
}

impl FieldForm {
    pub fn new(title: &str, fields: Vec<FormField>) -> Self {
        Self {
            title: title.to_string(),
            fields,
            selected: 0,
            error: None,
        }
    }

    pub fn value(&self, index: usize) -> &str {
        self.fields.get(index).map(|f| f.value.trim()).unwrap_or("")
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> FormResult {
        self.error = None;
        let count = self.fields.len().max(1);
        let field = &mut self.fields[self.selected];
        match key.code {
            KeyCode::Esc => return FormResult::Cancel,
            KeyCode::Enter => return FormResult::Submit,
            KeyCode::Tab | KeyCode::Down => self.selected = (self.selected + 1) % count,
            KeyCode::BackTab | KeyCode::Up => self.selected = (self.selected + count - 1) % count,
            KeyCode::Left if !field.choices.is_empty() => field.cycle(false),
            KeyCode::Right | KeyCode::Char(' ') if !field.choices.is_empty() => field.cycle(true),
            KeyCode::Backspace if field.choices.is_empty() => {
                field.value.pop();
            }
            KeyCode::Delete if field.choices.is_empty() => field.value.clear(),
            KeyCode::Char(c)
                if field.choices.is_empty() && !key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                field.value.push(c)
            }
            _ => {}
        }
        FormResult::Continue
    }

    pub fn render(&self, frame: &mut Frame, screen: Rect, pal: &Palette) {
        let label_width = self
            .fields
            .iter()
            .map(|f| UnicodeWidthStr::width(f.label.as_str()))
            .max()
            .unwrap_or(0) as u16;
        let width = (label_width + 46).min(screen.width.saturating_sub(4)).max(20);
        let height = (self.fields.len() as u16 + 6).min(screen.height);
        let area = Rect {
            x: screen.x + screen.width.saturating_sub(width) / 2,
            y: screen.y + screen.height.saturating_sub(height) / 2,
            width,
            height,
        };
        let inner = dm_box(frame, area, &self.title, pal, pal.fill(), true);
        let inner = inner.inner(&Margin {
            horizontal: 1,
            vertical: 0,
        });
        let value_width = inner.width.saturating_sub(label_width + 2) as usize;
        for (idx, field) in self.fields.iter().enumerate() {
            let row = idx as u16 + 1;
            if row >= inner.height {
                break;
            }
            let selected = idx == self.selected;
            let shown = if field.choices.is_empty() {
                // Show the end of long values so the cursor stays visible.
                let chars: Vec<char> = field.value.chars().collect();
                let skip = chars.len().saturating_sub(value_width.saturating_sub(1));
                chars[skip..].iter().collect::<String>()
            } else {
                format!("◄ {} ►", field.value)
            };
            let label = format!("{:>w$}: ", field.label, w = label_width as usize);
            let value_style = if selected { pal.selected() } else { pal.fill() };
            frame.render_widget(
                Paragraph::new(Line::from(vec![
                    Span::styled(label, pal.label()),
                    Span::styled(pad_width(&shown, value_width), value_style),
                ])),
                Rect {
                    x: inner.x,
                    y: inner.y + row,
                    width: inner.width,
                    height: 1,
                },
            );
            if selected && field.choices.is_empty() {
                let cursor = UnicodeWidthStr::width(shown.as_str()) as u16;
                frame.set_cursor(
                    inner.x + label_width + 2 + cursor.min(value_width as u16),
                    inner.y + row,
                );
            }
        }
        let footer_row = inner.height.saturating_sub(1);
        let footer = match &self.error {
            Some(err) => Span::styled(err.clone(), Style::default().fg(Color::Red).bg(pal.surface)),
            None => Span::styled(
                "Tab move · ←/→ choose · Enter save · Esc cancel",
                pal.fill().add_modifier(Modifier::ITALIC),
            ),
        };
        frame.render_widget(
            Paragraph::new(Line::from(footer)).alignment(Alignment::Center),
            Rect {
                x: inner.x,
                y: inner.y + footer_row,
                width: inner.width,
                height: 1,
            },
        );
    }
}

/// A centered yes/no question in a DeskMate box.
pub fn render_confirm(frame: &mut Frame, screen: Rect, message: &str, pal: &Palette) {
    let width = (UnicodeWidthStr::width(message) as u16 + 6)
        .max(36)
        .min(screen.width.saturating_sub(4));
    let area = Rect {
        x: screen.x + screen.width.saturating_sub(width) / 2,
        y: screen.y + screen.height.saturating_sub(6) / 2,
        width,
        height: 6.min(screen.height),
    };
    let inner = dm_box(frame, area, "Confirm", pal, pal.fill(), true);
    put_line(frame, inner, 1, &format!(" {message}"), pal.fill());
    put_line(frame, inner, 3, "   y / Enter = Yes      n / Esc = No", pal.fill().add_modifier(Modifier::ITALIC));
}
