use std::ops::Range;

#[derive(Debug)]
pub struct SkLine<'a> {
    pub content: &'a str,
    pub range: Range<usize>,
}

impl<'a> SkLine<'a> {
    pub fn new(i0: usize, i1: usize, content: &'a str) -> Self {
        Self {
            content,
            range: Range::from(i0..i1),
        }
    }

    pub fn column(&self, idx: usize) -> usize {
        idx - self.range.start
    }
}

#[derive(Debug)]
pub struct SkFile<'a> {
    lines: Vec<SkLine<'a>>,
    nlines: usize,
}

impl<'a> SkFile<'a> {
    pub fn new(src: &'a String) -> Self {
        let split_lines: Vec<&'a str> = src.split("\n").collect();
        let nlines = split_lines.len();

        let mut lines = Vec::<SkLine>::new();
        let mut last_line_start = 0usize;
        split_lines.iter().enumerate().for_each(|(i, x)| {
            last_line_start = i + 1;
            lines.push(SkLine::new(last_line_start, i, x))
        });

        Self {
            lines,
            nlines,
        }
    }

    pub fn pos(&self, idx: usize) -> Option<(usize, usize)> {
        for (i, x) in self.lines.iter().enumerate() {
            if x.range.contains(&idx) {
                return Some((x.column(idx), i));
            }
        }
        None
    }
}