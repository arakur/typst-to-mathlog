#[derive(Debug)]
pub struct Syntax {
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug)]
pub struct Paragraph {
    pub segments: Segments,
}

#[derive(Debug, Default)]
pub struct Segments(pub Vec<Segment>);

impl Segments {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, segment: Segment) {
        self.0.push(segment);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug)]
pub enum Segment {
    // Space,
    Linebreak,
    Heading(Heading),
    Text(Text),
    Strong(Strong),
    Emph(Emph),
    MathInline(MathInline),
    MathDisplay(MathDisplay),
    MathDelimited(MathDelimited),
    MathAttach(MathAttach),
    Command(Command),
    // TODO
}

#[derive(Debug)]
pub struct Heading {
    pub level: usize,
    pub content: Segments,
}

#[derive(Debug)]
pub struct Text(pub String);

#[derive(Debug)]
pub struct Strong {
    pub content: Segments,
}

#[derive(Debug)]
pub struct Emph {
    pub content: Segments,
}

#[derive(Debug)]
pub struct MathInline {
    pub content: Segments,
}

#[derive(Debug)]
pub struct MathDisplay {
    pub content: Segments,
}

#[derive(Debug)]
pub struct MathDelimited {
    pub open: Segments,
    pub body: Segments,
    pub close: Segments,
}

#[derive(Debug)]
pub struct MathAttach {
    pub base: Segments,
    pub top: Option<Segments>,
    pub bottom: Option<Segments>,
}

#[derive(Debug)]
pub struct Command(pub String);
