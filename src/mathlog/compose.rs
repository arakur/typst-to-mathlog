use super::ast::*;
use crate::utils::roman;

struct Composer {
    indent: usize,
    lines: Vec<String>,
    current_line: String,
}

impl Composer {
    fn new() -> Self {
        Self {
            indent: 0,
            lines: Vec::new(),
            current_line: String::new(),
        }
    }

    fn export(self) -> String {
        self.lines.join("\n") + "\n" + &self.current_line
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent -= 1;
    }

    fn newline(&mut self) {
        self.lines.push(std::mem::take(&mut self.current_line));
        self.current_line = String::new();
        self.current_line.push_str(&"    ".repeat(self.indent));
    }

    fn newline_if_not_empty(&mut self) {
        if !self.current_line.trim().is_empty() {
            self.newline();
        }
    }

    fn add(&mut self, s: &str) {
        self.current_line += s;
    }

    fn space(&mut self) {
        self.add(" ");
    }

    //

    fn paragraph(&mut self, paragraph: &Paragraph) {
        self.segments(&paragraph.segments);
    }

    fn segments(&mut self, segments: &Segments) {
        for (i, segment) in segments.0.iter().enumerate() {
            if i != 0 {
                // self.space();
            }
            self.segment(segment);
        }
    }

    fn segment(&mut self, segment: &Segment) {
        match segment {
            Segment::Linebreak => {
                self.add("\\\\");
                self.newline();
            }
            Segment::Heading(heading) => self.heading(heading),
            Segment::Text(text) => self.text(text),
            Segment::CodeInline(code) => self.code_inline(code),
            Segment::Strong(strong) => self.strong(strong),
            Segment::Emph(emph) => self.emph(emph),
            Segment::MathInline(math_inline) => self.math_inline(math_inline),
            Segment::MathDisplay(math_display) => self.math_display(math_display),
            Segment::ListItem(list) => self.list_item(list),
            Segment::MathDelimited(math_delimited) => self.math_delimited(math_delimited),
            Segment::MathAttach(math_attach) => self.math_attach(math_attach),
            Segment::MathAlignPoint => self.math_align_point(),
            Segment::Command(command) => self.command(command),
            Segment::RawCommand(command) => self.raw_command(command),
            Segment::Env(env) => self.env(env),
            Segment::ExportComment(comment) => self.export_comment(comment),
        }
    }

    fn heading(&mut self, heading: &Heading) {
        self.newline_if_not_empty();
        self.add(&format!("{} ", "#".repeat(heading.depth)));
        self.segments(&heading.content);
    }

    fn text(&mut self, text: &Text) {
        self.add(&text.0);
    }

    fn code_inline(&mut self, code: &CodeInline) {
        self.add(&code.0);
    }

    fn strong(&mut self, strong: &Strong) {
        self.add("**");
        self.segments(&strong.content);
        self.add("**");
    }

    fn emph(&mut self, emph: &Emph) {
        self.add("*");
        self.segments(&emph.content);
        self.add("*");
    }

    fn math_inline(&mut self, math_inline: &MathInline) {
        self.add("$");
        self.segments(&math_inline.content);
        self.add("$");
    }

    fn math_display(&mut self, math_display: &MathDisplay) {
        self.newline_if_not_empty();
        self.add("\\begin{align*}");
        self.newline();
        self.segments(&math_display.content);
        self.newline();
        self.add("\\end{align*}");
    }

    fn list_item(&mut self, list: &ListItem) {
        let symbol = match list.symbol {
            ListSymbol::NoNum => "-".to_string(),
            ListSymbol::NumDot(i) => format!("{}.", i),
            ListSymbol::NumParen(i) => format!("({})", i),
            ListSymbol::NumBrak(i) => format!("[{}]", i),
            ListSymbol::RomanDot(i) => format!("{}. ", roman(i)),
            ListSymbol::RomanParen(i) => format!("({}) ", roman(i)),
            ListSymbol::RomanBrak(i) => format!("[{}] ", roman(i)),
        };
        self.newline_if_not_empty();
        self.add(&symbol);
        self.space();
        self.indent();
        for (i, paragraph) in list.contents.iter().enumerate() {
            if i != 0 {
                self.newline();
            }
            self.paragraph(paragraph);
        }
        self.dedent();
    }

    fn math_delimited(&mut self, math_delimited: &MathDelimited) {
        // TODO
        let open = &math_delimited.open;
        let body = &math_delimited.body;
        let close = &math_delimited.close;
        self.add("\\left");
        self.segments(open);
        self.segments(body);
        self.add("\\right");
        self.segments(close);
    }

    fn math_attach(&mut self, math_attach: &MathAttach) {
        let base = &math_attach.base;
        let bottom = &math_attach.bottom;
        let top = &math_attach.top;
        self.segments(base);
        if let Some(bottom) = bottom {
            self.add("_");
            self.add("{");
            self.segments(bottom);
            self.add("}");
        }
        if let Some(top) = top {
            self.add("^");
            self.add("{");
            self.segments(top);
            self.add("}");
        }
    }

    fn math_align_point(&mut self) {
        self.add("&");
    }

    fn command(&mut self, command: &Command) {
        self.add("\\");
        self.add(&command.name);
        for arg in &command.args {
            if arg.is_optional {
                self.add("[");
            } else {
                self.add("{");
            }
            self.segments(&arg.content);
            if arg.is_optional {
                self.add("]");
            } else {
                self.add("}");
            }
        }
    }

    fn raw_command(&mut self, command: &RawCommand) {
        self.add(&command.0);
    }

    fn env(&mut self, env: &Env) {
        self.newline_if_not_empty();
        self.add("&&&");
        self.add(&env.kind.name());
        if let Some(title) = &env.title {
            self.space();
            self.segments(title);
        }
        self.newline();
        for (i, paragraph) in env.contents.iter().enumerate() {
            if i != 0 {
                self.newline();
            }
            self.paragraph(paragraph);
        }
        self.newline();
        self.add("&&&");
    }

    fn export_comment(&mut self, comment: &String) {
        self.newline_if_not_empty();
        self.add(&format!("<!-- {} -->", comment));
    }
}

impl Syntax {
    pub fn compose(&self) -> String {
        let mut composer = Composer::new();
        for (i, paragraph) in self.paragraphs.iter().enumerate() {
            if i != 0 {
                composer.newline();
                composer.newline();
            }
            composer.paragraph(paragraph);
        }
        composer.newline();
        composer.export()
    }
}
