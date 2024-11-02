use crate::border::BorderStyle;
use std::fmt;

pub struct Panel {
    content: String,
    title: Option<String>,
    style: BorderStyle,
    width: usize,
}

impl Panel {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            title: None,
            style: BorderStyle::default(),
            width: 40,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    fn contains_wide_chars(text: &str) -> bool {
        text.chars().any(|ch| {
            match ch as u32 {
                // Emoji & Pictographs
                0x1F300..=0x1F9FF |
                // Supplemental Symbols and Pictographs
                0x1FA70..=0x1FAFF |
                // Dingbats
                0x2700..=0x27BF |
                // Miscellaneous Symbols
                0x2600..=0x26FF |
                // CJK Symbols and Punctuation
                0x3000..=0x303F => true,
                _ => false
            }
        })
    }

    fn get_display_width(text: &str) -> usize {
        text.chars()
            .map(|ch| {
                if Self::contains_wide_chars(&ch.to_string()) {
                    2 // Count emoji and wide characters as width 2
                } else {
                    1 // Regular ASCII characters as width 1
                }
            })
            .sum()
    }

    fn wrap_text(&self, text: &str) -> Vec<String> {
        let max_width = self.width - 4; // Account for borders and padding
        text.lines()
            .flat_map(|line| {
                let mut wrapped = Vec::new();
                let mut current = String::new();
                let mut current_width = 0;

                for word in line.split_whitespace() {
                    let word_width = Self::get_display_width(word);
                    let space_width = if current_width > 0 { 1 } else { 0 };

                    if current_width + word_width + space_width <= max_width {
                        if current_width > 0 {
                            current.push(' ');
                            current_width += 1;
                        }
                        current.push_str(word);
                        current_width += word_width;
                    } else {
                        if !current.is_empty() {
                            wrapped.push(current);
                        }
                        current = word.to_string();
                        current_width = word_width;
                    }
                }
                if !current.is_empty() {
                    wrapped.push(current);
                }
                if wrapped.is_empty() && !line.is_empty() {
                    wrapped.push(line.to_string());
                }
                wrapped
            })
            .collect()
    }
}

impl fmt::Display for Panel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = self.style.get_chars();
        let inner_width = self.width - 2;

        // Top border with optional title
        write!(f, "{}", chars.top_left)?;
        if let Some(title) = &self.title {
            let title = if title.len() > inner_width - 4 {
                format!("{}...", &title[..inner_width - 7])
            } else {
                title.clone()
            };
            let padding = inner_width - title.len() - 4;
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            write!(
                f,
                "{}[ {} ]{}",
                chars.horizontal.to_string().repeat(left_pad),
                title,
                chars.horizontal.to_string().repeat(right_pad)
            )?;
        } else {
            write!(f, "{}", chars.horizontal.to_string().repeat(inner_width))?;
        }
        writeln!(f, "{}", chars.top_right)?;

        // Content with proper padding
        let wrapped_lines = self.wrap_text(&self.content);
        for line in wrapped_lines {
            write!(f, "{} ", chars.vertical)?;
            let display_width = Self::get_display_width(&line);
            let padding = inner_width - 2 - display_width;
            write!(f, "{}", line)?;
            write!(f, "{:padding$}", "", padding = padding)?;
            writeln!(f, " {}", chars.vertical)?;
        }

        // Bottom border
        write!(f, "{}", chars.bottom_left)?;
        write!(f, "{}", chars.horizontal.to_string().repeat(inner_width))?;
        writeln!(f, "{}", chars.bottom_right)
    }
}
