#[derive(Debug, Clone, Copy, Default)]
pub enum BorderStyle {
    #[default]
    Single,
    Double,
    Rounded,
}

pub(crate) struct BorderChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub left_separator: char,
    pub right_separator: char,
    pub top_separator: char,
    pub bottom_separator: char,
    pub cross: char,
}

impl BorderStyle {
    pub(crate) fn get_chars(&self) -> BorderChars {
        match self {
            Self::Single => BorderChars {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
                left_separator: '├',
                right_separator: '┤',
                top_separator: '┬',
                bottom_separator: '┴',
                cross: '┼',
            },
            Self::Double => BorderChars {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
                left_separator: '╠',
                right_separator: '╣',
                top_separator: '╦',
                bottom_separator: '╩',
                cross: '╬',
            },
            Self::Rounded => BorderChars {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
                left_separator: '├',
                right_separator: '┤',
                top_separator: '┬',
                bottom_separator: '┴',
                cross: '┼',
            },
        }
    }
}
