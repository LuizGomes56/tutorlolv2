//! Renders a `.ir` + `.txt` pair back into the highlighted HTML output.

use crate::dictionary::Dictionary;
use crate::header::{Header, HEADER_SIZE};
use crate::io::Cursor;
use crate::segment::Segment;
use crate::template::TemplateRegistry;

pub struct Decoder<'a> {
    header: Header,
    cursor: Cursor<'a>,
    dictionary: &'a Dictionary,
    templates: &'a TemplateRegistry,
}

impl<'a> Decoder<'a> {
    /// Parses the header out of `ir` and prepares to read the segments
    /// that follow it, resolving unknown tokens against `aux`.
    pub fn new(ir: &'a [u8], aux: &'a [u8], dictionary: &'a Dictionary, templates: &'a TemplateRegistry) -> Self {
        let header = Header::read_from(ir).expect("malformed .ir file: bad magic or truncated header");
        let cursor = Cursor::new(&ir[HEADER_SIZE..], aux);
        Decoder { header, cursor, dictionary, templates }
    }

    /// Reads and renders every segment declared in the header, in order.
    pub fn render(mut self) -> String {
        let mut html = String::new();
        for _ in 0..self.header.segment_count {
            let segment = Segment::read(&mut self.cursor, self.templates);
            html.push_str(&segment.render(self.dictionary, self.templates));
        }
        html
    }
}
