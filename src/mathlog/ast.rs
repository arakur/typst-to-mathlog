#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Syntax {
    pub paragraphs: Paragraphs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraphs(pub Vec<Paragraph>);

impl Paragraphs {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, paragraph: Paragraph) {
        self.0.push(paragraph)
    }
}

impl Default for Paragraphs {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph {
    pub lines: Vec<Line>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Heading(Heading),
    Text(Text),
    Env(Env),
    Enum(Enum),
    Quote(Quote),
    Code(CodeBlock),
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub level: usize,
    pub segments: Segments,
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub segments: Segments,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Segment {
    Plain(String),
    MathInline(Math),
    MathDisplay(Math),
    HTMLTag(HTMLTag),
    Emph(Emph),
    Link(Link),
    Table(Table),
    Image(Image),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segments(pub Vec<Segment>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Math {
    pub segments: MathSegments,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MathSegments(pub Vec<MathSegment>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathSegment {
    Single(String),
    Braced {
        segments: MathSegments,
    },
    Delimited {
        left: String,
        segments: MathSegments,
        right: String,
    },
    Function {
        name: String,
        arguments: Vec<MathArg>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathArg {
    Optional(Math),
    Required(Math),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTMLTag {
    pub name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Paragraphs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emph {
    pub kind: EmphKind,
    pub child: Paragraph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmphKind {
    Bold,
    Italic,
    Strike,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub text: String,
    pub url: String,
}

// TODO: support all options
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table {
    pub header: TableRow,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRow {
    pub cells: Vec<Segments>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub alt: String,
    pub url: String,
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env {
    pub kind: EnvKind,
    pub title: Option<Segments>,
    pub children: Paragraphs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnvKind {
    Block,
    Conj,
    Axm,
    Def,
    Prop,
    Fml,
    Lem,
    Thm,
    Cor,
    Prf,
    Ex,
    Exc,
    Rem,
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub kind: EnumKind,
    pub children: Vec<Paragraph>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumKind {
    NoNum,
    Num {
        num_kind: NumKind,
        style: EnumNumStyle,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumKind {
    Arabic,
    Roman,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumNumStyle {
    Period,
    Paren,
    Square,
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Quote {
    pub children: Paragraphs,
}

//

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeBlock {
    pub lang: Option<String>,
    pub code: String,
}
