use super::ast::*;

impl Syntax {
    pub fn compose(&self) -> String {
        self.paragraphs
            .iter()
            .map(|paragraph| paragraph.compose())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl Paragraph {
    fn compose(&self) -> String {
        self.segments.compose()
    }
}

impl Segments {
    fn compose(&self) -> String {
        self.0
            .iter()
            .map(|segment| segment.compose())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Segment {
    fn compose(&self) -> String {
        match self {
            // Segment::Space => " ".to_string(),
            Segment::Linebreak => "\n".to_string(),
            Segment::Heading(heading) => heading.compose(),
            Segment::Text(text) => text.compose(),
            Segment::Strong(strong) => strong.compose(),
            Segment::Emph(emph) => emph.compose(),
            Segment::MathInline(equation) => equation.compose(),
            Segment::MathDisplay(equation) => equation.compose(),
            Segment::MathDelimited(math_delimited) => math_delimited.compose(),
            Segment::MathAttach(math_attach) => math_attach.compose(),
            Segment::Command(math_command) => math_command.compose(),
        }
    }
}

impl Heading {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "{} {}",
            "#".repeat(self.level),
            self.content.compose()
        ));
        output
    }
}

impl Text {
    fn compose(&self) -> String {
        self.0.clone()
    }
}

impl Strong {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("**{}**", self.content.compose()));
        output
    }
}

impl Emph {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("*{}*", self.content.compose()));
        output
    }
}

impl MathInline {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("${}$", self.content.compose()));
        output
    }
}

impl MathDisplay {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("$$\n    {}\n$$", self.content.compose()));
        output
    }
}

impl MathDelimited {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "\\left{} {} \\right{}",
            self.open.compose(),
            self.body.compose(),
            self.close.compose()
        ));
        output
    }
}

impl MathAttach {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.base.compose());
        if let Some(top) = &self.top {
            output.push('^');
            output.push_str(&top.compose());
        }
        if let Some(bottom) = &self.bottom {
            output.push('_');
            output.push_str(&bottom.compose());
        }
        output
    }
}

impl Command {
    fn compose(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.0);
        output
    }
}
