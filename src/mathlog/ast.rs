#[derive(Debug, Clone)]
pub struct Syntax {
    pub paragraphs: Paragraphs,
}

pub type Paragraphs = Vec<Paragraph>;

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub segments: Segments,
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub enum Segment {
    // Space,
    Linebreak,
    Heading(Heading),
    Text(Text),
    CodeInline(CodeInline),
    Strong(Strong),
    Emph(Emph),
    MathInline(MathInline),
    MathDisplay(MathDisplay),
    ListItem(ListItem),
    MathDelimited(MathDelimited),
    MathAttach(MathAttach),
    MathAlignPoint,
    Command(Command),
    RawCommand(RawCommand),
    Env(Env),
    ExportComment(String),
    // TODO
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: usize,
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct Text(pub String);

#[derive(Debug, Clone)]
pub struct CodeInline(pub String);

#[derive(Debug, Clone)]
pub struct Strong {
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct Emph {
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct MathInline {
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct MathDisplay {
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub symbol: ListSymbol,
    pub contents: Paragraphs,
}

#[derive(Debug, Clone)]
pub enum ListSymbol {
    NoNum,
    NumDot(usize),
    NumParen(usize),
    NumBrak(usize),
    RomanDot(usize),
    RomanParen(usize),
    RomanBrak(usize),
}

#[derive(Debug, Clone)]
pub struct MathDelimited {
    pub open: Segments,
    pub body: Segments,
    pub close: Segments,
}

#[derive(Debug, Clone)]
pub struct MathAttach {
    pub base: Segments,
    pub top: Option<Segments>,
    pub bottom: Option<Segments>,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub kind: EnvKind,
    pub title: Option<Segments>,
    pub contents: Paragraphs,
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

impl EnvKind {
    pub const LIST: [(EnvKind, &str); 13] = [
        (EnvKind::Block, ""),
        (EnvKind::Conj, "conj"),
        (EnvKind::Axm, "axm"),
        (EnvKind::Def, "def"),
        (EnvKind::Prop, "prop"),
        (EnvKind::Fml, "fml"),
        (EnvKind::Lem, "lem"),
        (EnvKind::Thm, "thm"),
        (EnvKind::Cor, "cor"),
        (EnvKind::Prf, "prf"),
        (EnvKind::Ex, "ex"),
        (EnvKind::Exc, "exc"),
        (EnvKind::Rem, "rem"),
    ];

    pub fn name(&self) -> String {
        Self::LIST
            .iter()
            .find(|(kind, _)| kind == self)
            .map(|(_, name)| name.to_string())
            .unwrap_or_else(|| panic!("unexpected env kind: {:?}", self))
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::LIST
            .iter()
            .find(|(_, n)| n == &name)
            .map(|(kind, _)| kind.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub is_optional: bool,
    pub content: Segments,
}

#[derive(Debug, Clone)]
pub struct RawCommand(pub String);
