#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl Alignment {
    pub(crate) fn format_content(&self, content: &str, width: usize) -> String {
        let truncated = if content.len() > width {
            format!("{}...", &content[..width.saturating_sub(3)])
        } else {
            content.to_string()
        };

        match self {
            Self::Left => format!(" {:<width$} ", truncated, width = width),
            Self::Right => format!(" {:>width$} ", truncated, width = width),
            Self::Center => format!(" {:^width$} ", truncated, width = width),
        }
    }
}
