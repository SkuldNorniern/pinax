use crate::border::BorderStyle;
use std::fmt;

pub struct Grid {
    cells: Vec<Vec<String>>,
    rows: usize,
    cols: usize,
    style: BorderStyle,
}

#[derive(Default)]
pub struct GridBuilder {
    rows: usize,
    cols: usize,
    style: Option<BorderStyle>,
}

impl GridBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dimensions(mut self, rows: usize, cols: usize) -> Self {
        self.rows = rows;
        self.cols = cols;
        self
    }

    pub fn style(mut self, style: BorderStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Grid {
        Grid::new(self.rows, self.cols, self.style.unwrap_or_default())
    }
}

impl Grid {
    pub fn builder() -> GridBuilder {
        GridBuilder::new()
    }

    fn new(rows: usize, cols: usize, style: BorderStyle) -> Self {
        let cells = vec![vec![String::new(); cols]; rows];
        Self {
            cells,
            rows,
            cols,
            style,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: impl Into<String>) {
        if row < self.rows && col < self.cols {
            self.cells[row][col] = value.into();
        }
    }

    fn write_horizontal_border(
        &self,
        f: &mut fmt::Formatter<'_>,
        left: char,
        right: char,
        horizontal: char,
        separator: char,
    ) -> fmt::Result {
        write!(f, "{}", left)?;
        for i in 0..self.cols {
            write!(f, "{}", horizontal.to_string().repeat(20))?;
            if i < self.cols - 1 {
                write!(f, "{}", separator)?;
            }
        }
        writeln!(f, "{}", right)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = self.style.get_chars();

        self.write_horizontal_border(
            f,
            chars.top_left,
            chars.top_right,
            chars.horizontal,
            chars.top_separator,
        )?;

        for (i, row) in self.cells.iter().enumerate() {
            write!(f, "{}", chars.vertical)?;
            for (j, cell) in row.iter().enumerate() {
                write!(f, "{:^20}", cell)?;
                if j < self.cols - 1 {
                    write!(f, "{}", chars.vertical)?;
                }
            }
            writeln!(f, "{}", chars.vertical)?;

            if i < self.rows - 1 {
                self.write_horizontal_border(
                    f,
                    chars.left_separator,
                    chars.right_separator,
                    chars.horizontal,
                    chars.cross,
                )?;
            }
        }

        self.write_horizontal_border(
            f,
            chars.bottom_left,
            chars.bottom_right,
            chars.horizontal,
            chars.bottom_separator,
        )
    }
}
