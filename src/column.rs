use crate::alignment::Alignment;

const MIN_PADDING: usize = 2;
const DEFAULT_PADDING: usize = 4;

#[derive(Clone)]
pub struct Column {
    pub(crate) header: String,
    pub(crate) alignment: Alignment,
    pub(crate) min_width: usize,
    pub(crate) max_width: Option<usize>,
    pub(crate) padding: usize,
}

impl Column {
    pub fn new(header: impl Into<String>) -> Self {
        let header = header.into();
        Self {
            min_width: header.len(),
            header,
            alignment: Alignment::Left,
            max_width: None,
            padding: DEFAULT_PADDING,
        }
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_width(mut self, min_width: usize) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding.max(MIN_PADDING);
        self
    }

    pub(crate) fn get_content_width(&self, content_width: usize) -> usize {
        let width = content_width.max(self.min_width);
        match self.max_width {
            Some(max) => width.min(max),
            None => width,
        }
    }

    pub(crate) fn format_content(&self, content: &str) -> String {
        let width = self.get_content_width(content.len());
        self.alignment.format_content(content, width)
    }

    pub(crate) fn total_width(&self) -> usize {
        self.min_width + self.padding
    }
}
