use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ChartType {
    Bar,
    Line,
}

pub struct Chart {
    data: Vec<(String, f64)>,
    chart_type: ChartType,
    height: usize,
    title: Option<String>,
}

impl Chart {
    pub fn new(chart_type: ChartType) -> Self {
        Self {
            data: Vec::new(),
            chart_type,
            height: 10,
            title: None,
        }
    }

    pub fn add_data_point(&mut self, label: impl Into<String>, value: f64) {
        self.data.push((label.into(), value));
    }

    pub fn with_height(mut self, height: usize) -> Self {
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    fn generate_bar_chart(&self) -> Vec<String> {
        let max_value = self.data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
        let mut output = Vec::new();

        // Generate bars with fixed width and proper alignment
        for (label, value) in &self.data {
            let bar_width = ((value / max_value) * 40.0) as usize;
            let bar = "█".repeat(bar_width);
            output.push(format!("{:<6} │ {:<40} {:.1}", label, bar, value));
        }

        output
    }

    fn generate_line_chart(&self) -> Vec<String> {
        if self.data.is_empty() {
            return Vec::new();
        }

        let max_value = self.data.iter().map(|(_, v)| *v).fold(0.0, f64::max);
        let min_value = self.data.iter().map(|(_, v)| *v).fold(f64::MAX, f64::min);
        let col_width = 12; // Width for each column
        let width = self.data.len() * col_width;
        let mut chart = vec![vec![' '; width]; self.height];

        // Calculate y-axis values for key points
        let y_range = max_value - min_value;
        let y_values = [
            max_value,                        // Top
            max_value - (y_range / 2.0),      // Middle
            min_value,                        // Bottom
        ];

        // Plot points and lines
        for i in 0..self.data.len() - 1 {
            let (x1, y1) = (
                i * col_width,
                self.height - 1 - ((self.data[i].1 - min_value) / (max_value - min_value) * (self.height - 1) as f64) as usize
            );
            let (x2, y2) = (
                (i + 1) * col_width,
                self.height - 1 - ((self.data[i + 1].1 - min_value) / (max_value - min_value) * (self.height - 1) as f64) as usize
            );

            // Draw points
            chart[y1][x1] = '●';
            if i == self.data.len() - 2 {
                chart[y2][x2] = '●';
            }

            // Draw connecting line
            let dx = x2 - x1;
            let dy = y2 as i32 - y1 as i32;
            let steps = dx.max(dy.abs() as usize);
            
            for step in 1..steps {
                let x = x1 + step;
                let y = y1 as i32 + (dy * step as i32 / steps as i32);
                if y >= 0 && y < self.height as i32 {
                    // Choose character based on line direction
                    let char = if dy == 0 {
                        '─'
                    } else if dy < 0 {
                        '/'
                    } else {
                        '\\'
                    };
                    chart[y as usize][x] = char;
                }
            }
        }

        // Convert to strings with axis and y-values
        let mut output = Vec::new();
        for (i, row) in chart.iter().enumerate() {
            let y_value = if i == 0 {
                format!("{:>8.1} ", y_values[0])
            } else if i == self.height / 2 {
                format!("{:>8.1} ", y_values[1])
            } else if i == self.height - 1 {
                format!("{:>8.1} ", y_values[2])
            } else {
                format!("{:<10}", "")
            };
            output.push(format!("{:<10}│ {}", y_value, row.iter().collect::<String>()));
        }

        // Add x-axis
        output.push(format!("{:<10}└{}", "", "─".repeat(width)));

        // Add labels with improved spacing
        let mut labels = String::from("            ");
        for (i, (label, _)) in self.data.iter().enumerate() {
            labels.push_str(&format!("{:<11}", label));
            if i < self.data.len() - 1 {
                labels.push(' ');
            }
        }
        output.push(labels);

        output
    }
}

impl fmt::Display for Chart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write title if present
        if let Some(title) = &self.title {
            writeln!(f, "{}", title)?;
            writeln!(f, "{}", "=".repeat(title.len()))?;
            writeln!(f)?; // Add extra newline after title
        }

        let chart_data = match self.chart_type {
            ChartType::Bar => self.generate_bar_chart(),
            ChartType::Line => self.generate_line_chart(),
        };

        for line in chart_data {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}
