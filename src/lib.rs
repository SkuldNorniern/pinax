#[derive(Debug, Clone, Copy)]
pub enum BorderStyle {
    Single,
    Double,
    Rounded,
}

impl BorderStyle {
    fn get_chars(&self) -> BorderChars {
        match self {
            BorderStyle::Single => BorderChars {
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
            BorderStyle::Double => BorderChars {
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
            BorderStyle::Rounded => BorderChars {
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

struct BorderChars {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
    left_separator: char,
    right_separator: char,
    top_separator: char,
    bottom_separator: char,
    cross: char,
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

pub struct Column {
    header: String,
    alignment: Alignment,
    width: Option<usize>,
}

impl Column {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            alignment: Alignment::Left,
            width: None,
        }
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    style: BorderStyle,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn new(columns: Vec<Column>) -> Self {
        let column_widths = columns.iter()
            .map(|col| col.width.unwrap_or_else(|| col.header.len()))
            .collect();

        Table {
            columns,
            rows: Vec::new(),
            style: BorderStyle::Double,
            column_widths,
        }
    }

    pub fn with_style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }

    pub fn add_row(&mut self, row: Vec<impl Into<String>>) {
        let row: Vec<String> = row.into_iter().map(Into::into).collect();
        
        // Update column widths if necessary
        for (i, item) in row.iter().enumerate() {
            if i < self.column_widths.len() {
                self.column_widths[i] = self.column_widths[i].max(item.len());
            }
        }
        self.rows.push(row);
    }

    fn format_cell(&self, content: &str, width: usize, alignment: Alignment) -> String {
        match alignment {
            Alignment::Left => format!(" {:<width$} ", content, width = width),
            Alignment::Right => format!(" {:>width$} ", content, width = width),
            Alignment::Center => format!(" {:^width$} ", content, width = width),
        }
    }

    pub fn print(&self) {
        let chars = self.style.get_chars();
        
        // Print top border
        self.print_horizontal_border(chars.top_left, chars.top_right, chars.horizontal, chars.top_separator);
        
        // Print headers
        print!("{}", chars.vertical);
        for (i, (col, width)) in self.columns.iter().zip(self.column_widths.iter()).enumerate() {
            print!("{}", self.format_cell(&col.header, *width, col.alignment));
            if i < self.columns.len() - 1 {
                print!("{}", chars.vertical);
            }
        }
        println!("{}", chars.vertical);
        
        // Print separator
        self.print_horizontal_border(chars.left_separator, chars.right_separator, chars.horizontal, chars.cross);
        
        // Print data rows
        for row in &self.rows {
            print!("{}", chars.vertical);
            for (i, ((item, width), col)) in row.iter()
                .zip(self.column_widths.iter())
                .zip(self.columns.iter())
                .enumerate() 
            {
                print!("{}", self.format_cell(item, *width, col.alignment));
                if i < row.len() - 1 {
                    print!("{}", chars.vertical);
                }
            }
            println!("{}", chars.vertical);
        }
        
        // Print bottom border
        self.print_horizontal_border(chars.bottom_left, chars.bottom_right, chars.horizontal, chars.bottom_separator);
    }

    fn print_horizontal_border(&self, left: char, right: char, horizontal: char, separator: char) {
        print!("{}", left);
        for (i, width) in self.column_widths.iter().enumerate() {
            print!("{}", horizontal.to_string().repeat(width + 2));
            if i < self.column_widths.len() - 1 {
                print!("{}", separator);
            }
        }
        println!("{}", right);
    }
}