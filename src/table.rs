use std::fmt;
use crate::{alignment::Alignment, border::BorderStyle, column::Column};

#[derive(Default)]
pub struct TableBuilder {
    columns: Vec<Column>,
    style: Option<BorderStyle>,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_column(mut self, column: Column) -> Self {
        self.columns.push(column);
        self
    }

    pub fn style(mut self, style: BorderStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> Table {
        Table::new(self.columns, self.style.unwrap_or_default())
    }
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    style: BorderStyle,
    column_widths: Vec<usize>,
}

impl Table {
    pub fn builder() -> TableBuilder {
        TableBuilder::new()
    }

    fn new(columns: Vec<Column>, style: BorderStyle) -> Self {
        let column_widths = columns.iter()
            .map(|col| col.total_width())
            .collect();

        Self {
            columns,
            rows: Vec::new(),
            style,
            column_widths,
        }
    }

    fn calculate_column_widths(&mut self) {
        self.column_widths = self.columns.iter().enumerate()
            .map(|(i, col)| {
                let content_widths = std::iter::once(&col.header)
                    .chain(self.rows.iter().filter_map(|row| row.get(i)))
                    .map(|s| s.len());
                
                let max_content = content_widths.max().unwrap_or(0);
                col.get_content_width(max_content) + col.padding
            })
            .collect();
    }

    pub fn add_row(&mut self, row: Vec<impl Into<String>>) {
        let row: Vec<String> = row.into_iter().map(Into::into).collect();
        self.rows.push(row);
        self.calculate_column_widths();
    }

    fn write_row(&self, f: &mut fmt::Formatter<'_>, row: &[String], chars: &crate::border::BorderChars) -> fmt::Result {
        write!(f, "{}", chars.vertical)?;
        
        for (i, ((item, width), col)) in row.iter()
            .zip(&self.column_widths)
            .zip(&self.columns)
            .enumerate() 
        {
            let formatted = col.format_content(item);
            write!(f, "{:width$}", formatted, width = width)?;
            
            if i < row.len() - 1 {
                write!(f, "{}", chars.vertical)?;
            }
        }
        
        writeln!(f, "{}", chars.vertical)
    }

    fn write_horizontal_border(
        &self,
        f: &mut fmt::Formatter<'_>,
        left: char,
        right: char,
        horizontal: char,
        separator: char
    ) -> fmt::Result {
        write!(f, "{}", left)?;
        
        for (i, width) in self.column_widths.iter().enumerate() {
            write!(f, "{}", horizontal.to_string().repeat(*width))?;
            if i < self.column_widths.len() - 1 {
                write!(f, "{}", separator)?;
            }
        }
        
        writeln!(f, "{}", right)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chars = self.style.get_chars();
        
        self.write_horizontal_border(f, chars.top_left, chars.top_right, chars.horizontal, chars.top_separator)?;
        
        let headers: Vec<String> = self.columns.iter()
            .map(|col| col.header.clone())
            .collect();
        self.write_row(f, &headers, &chars)?;
        
        self.write_horizontal_border(f, chars.left_separator, chars.right_separator, chars.horizontal, chars.cross)?;
        
        for row in &self.rows {
            self.write_row(f, row, &chars)?;
        }
        
        self.write_horizontal_border(f, chars.bottom_left, chars.bottom_right, chars.horizontal, chars.bottom_separator)
    }
}
