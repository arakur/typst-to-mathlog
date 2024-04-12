use super::ast as mathlog;
use super::*;
use typst::syntax as typst;

use core::fmt;

//

#[derive(Debug)]
pub enum FromTypstErrorKind {
    UnexpectedNode(String),
    UnsupportedNode(String),
    UnsupportedIdent(Vec<String>),
    UnsupportedModule(Vec<String>),
    UnsupportedFuncCall(Vec<String>),
    EnvInSegments(String),
    NotYetImplemented(String),
}

impl fmt::Display for FromTypstErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromTypstErrorKind::UnexpectedNode(s) => {
                write!(f, "unexpected node: {}", s)
            }
            FromTypstErrorKind::UnsupportedNode(s) => {
                write!(f, "unsupported node: {}", s)
            }
            FromTypstErrorKind::UnsupportedIdent(s) => {
                write!(f, "unsupported ident: {}", s.join("."))
            }
            FromTypstErrorKind::UnsupportedModule(s) => {
                write!(f, "unsupported module: {}", s.join("."))
            }
            FromTypstErrorKind::UnsupportedFuncCall(s) => {
                write!(f, "unsupported function call: {}", s.join("."))
            }
            FromTypstErrorKind::EnvInSegments(s) => write!(
                f,
                "it is unsupported for an environment {} to be in a line",
                s
            ),
            FromTypstErrorKind::NotYetImplemented(s) => {
                write!(f, "not yet implemented: {}", s)
            }
        }
    }
}

#[derive(Debug)]
pub struct FromTypstError {
    pub kind: FromTypstErrorKind,
}

impl fmt::Display for FromTypstError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FromTypstError: {}", self.kind)
    }
}

impl std::error::Error for FromTypstError {}

pub type FromTypstResult<T> = Result<T, FromTypstError>;

impl FromTypstError {
    pub fn unexpected_node(node_kind: &str) -> Self {
        Self {
            kind: FromTypstErrorKind::UnexpectedNode(node_kind.to_string()),
        }
    }

    pub fn unsupported_node(node_kind: &str) -> Self {
        Self {
            kind: FromTypstErrorKind::UnsupportedNode(node_kind.to_string()),
        }
    }

    pub fn unsupported_ident(path: Vec<String>) -> Self {
        Self {
            kind: FromTypstErrorKind::UnsupportedIdent(path),
        }
    }

    pub fn unsupported_module(path: Vec<String>) -> Self {
        Self {
            kind: FromTypstErrorKind::UnsupportedModule(path),
        }
    }

    pub fn unsupported_func_call(path: Vec<String>) -> Self {
        Self {
            kind: FromTypstErrorKind::UnsupportedFuncCall(path),
        }
    }

    pub fn env_in_segments(s: String) -> Self {
        Self {
            kind: FromTypstErrorKind::EnvInSegments(s),
        }
    }

    pub fn not_yet_implemented(s: String) -> Self {
        Self {
            kind: FromTypstErrorKind::NotYetImplemented(s),
        }
    }
}

//

struct SegmentWriter {
    segments: mathlog::Segments,
}

impl SegmentWriter {
    fn new() -> Self {
        Self {
            segments: mathlog::Segments::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    fn push_segment(&mut self, segment: mathlog::Segment) {
        self.segments.push(segment);
    }

    // fn push_linebreak(&mut self) {
    //     self.segments.push(mathlog::Segment::Linebreak);
    // }

    fn export(self) -> mathlog::Segments {
        // NOTE: no process is needed when returning segments
        self.segments
    }
}

impl Default for SegmentWriter {
    fn default() -> Self {
        Self::new()
    }
}

struct ParagraphWriter {
    paragraphs: Vec<mathlog::Paragraph>,
    segments_writer: SegmentWriter,
}

impl ParagraphWriter {
    fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
            segments_writer: SegmentWriter::new(),
        }
    }

    fn push_segment(&mut self, segment: mathlog::Segment) {
        self.segments_writer.push_segment(segment);
    }

    // fn push_linebreak(&mut self) {
    //     self.segments_writer.push_linebreak();
    // }

    fn push_paragraph(&mut self) {
        let segments = std::mem::take(&mut self.segments_writer).export();
        self.paragraphs.push(mathlog::Paragraph { segments });
    }

    fn push_paragraph_if_not_empty(&mut self) {
        if !self.segments_writer.is_empty() {
            self.push_paragraph();
        }
    }

    fn export(mut self) -> Vec<mathlog::Paragraph> {
        self.push_paragraph_if_not_empty();
        self.paragraphs
    }
}

//

impl mathlog::Syntax {
    pub fn from_typst(node: &typst::ast::Markup, dic: &Dictionary) -> FromTypstResult<Self> {
        let mut writer = ParagraphWriter::new();
        writer.markup(node, dic)?;
        let paragraphs = writer.export();
        Ok(Self { paragraphs })
    }
}

//

impl ParagraphWriter {
    fn markup(&mut self, node: &typst::ast::Markup, dic: &Dictionary) -> FromTypstResult<()> {
        for node in node.exprs() {
            self.expr(&node, dic)?;
        }
        Ok(())
    }

    fn expr(&mut self, node: &typst::ast::Expr, dic: &Dictionary) -> FromTypstResult<()> {
        match node {
            typst::ast::Expr::Parbreak(parbreak) => self.parbreak(parbreak),
            typst::ast::Expr::Heading(heading) => self.heading(heading, dic),
            typst::ast::Expr::Space(space) => self.segments_writer.space(space),
            typst::ast::Expr::Text(text) => self.segments_writer.text(text),
            typst::ast::Expr::Strong(strong) => self.segments_writer.strong(strong, dic),
            typst::ast::Expr::Emph(emph) => self.segments_writer.emph(emph, dic),
            typst::ast::Expr::Equation(equation) => self.equation(equation, dic),
            typst::ast::Expr::List(list) => self.list_item(list, dic),
            typst::ast::Expr::Enum(enum_item) => self.enum_item(enum_item, dic),
            typst::ast::Expr::Show(show) => self.show(show),
            typst::ast::Expr::FuncCall(func_call) => self.func_call(func_call, dic),
            typst::ast::Expr::Import(import) => self.import(import),
            typst::ast::Expr::Include(include) => self.include(include),
            typst::ast::Expr::Linebreak(linebreak) => self.segments_writer.linebreak(linebreak),
            typst::ast::Expr::Escape(escape) => self.segments_writer.escape(escape),
            typst::ast::Expr::Shorthand(shorthand) => self.segments_writer.shorthand(shorthand),
            typst::ast::Expr::SmartQuote(quote) => self.segments_writer.smart_quote(quote),
            typst::ast::Expr::Raw(raw) => self.segments_writer.raw(raw),
            typst::ast::Expr::Link(link) => self.link(link),
            typst::ast::Expr::Label(label) => self.label(label),
            typst::ast::Expr::Ref(ref_) => self.ref_(ref_),
            typst::ast::Expr::Term(term) => self.term(term),
            //
            typst::ast::Expr::Math(_) => Err(FromTypstError::unexpected_node("math")),
            typst::ast::Expr::MathIdent(_) => Err(FromTypstError::unexpected_node("math ident")),
            typst::ast::Expr::MathAlignPoint(_) => {
                Err(FromTypstError::unexpected_node("math align point"))
            }
            typst::ast::Expr::MathDelimited(_) => {
                Err(FromTypstError::unexpected_node("math delimited"))
            }
            typst::ast::Expr::MathAttach(_) => Err(FromTypstError::unexpected_node("math attach")),
            typst::ast::Expr::MathFrac(_) => Err(FromTypstError::unexpected_node("math frac")),
            typst::ast::Expr::MathRoot(_) => Err(FromTypstError::unexpected_node("math root")),
            typst::ast::Expr::Ident(_) => Err(FromTypstError::unexpected_node("ident")),
            typst::ast::Expr::None(_) => Err(FromTypstError::unexpected_node("none")),
            typst::ast::Expr::Auto(_) => Err(FromTypstError::unexpected_node("auto")),
            typst::ast::Expr::Bool(_) => Err(FromTypstError::unexpected_node("bool")),
            typst::ast::Expr::Int(_) => Err(FromTypstError::unexpected_node("int")),
            typst::ast::Expr::Float(_) => Err(FromTypstError::unexpected_node("float")),
            typst::ast::Expr::Numeric(_) => Err(FromTypstError::unexpected_node("numeric")),
            typst::ast::Expr::Str(_) => Err(FromTypstError::unexpected_node("str")),
            typst::ast::Expr::Code(_) => Err(FromTypstError::unexpected_node("code")),
            typst::ast::Expr::Content(_) => Err(FromTypstError::unexpected_node("content")),
            typst::ast::Expr::Parenthesized(_) => {
                Err(FromTypstError::unexpected_node("parenthesized"))
            }
            typst::ast::Expr::Array(_) => Err(FromTypstError::unexpected_node("array")),
            typst::ast::Expr::Dict(_) => Err(FromTypstError::unexpected_node("dict")),
            typst::ast::Expr::Unary(_) => Err(FromTypstError::unexpected_node("unary")),
            typst::ast::Expr::Binary(_) => Err(FromTypstError::unexpected_node("binary")),
            typst::ast::Expr::FieldAccess(_) => {
                Err(FromTypstError::unexpected_node("field access"))
            }
            typst::ast::Expr::Closure(_) => Err(FromTypstError::unexpected_node("closure")),
            typst::ast::Expr::Let(_) => Err(FromTypstError::unsupported_node("let")),
            typst::ast::Expr::DestructAssign(_) => {
                Err(FromTypstError::unexpected_node("destruct assign"))
            }
            typst::ast::Expr::Set(_) => Err(FromTypstError::unsupported_node("set")),
            typst::ast::Expr::Conditional(_) => Err(FromTypstError::unexpected_node("conditional")),
            typst::ast::Expr::While(_) => Err(FromTypstError::unexpected_node("while")),
            typst::ast::Expr::For(_) => Err(FromTypstError::unexpected_node("for")),
            typst::ast::Expr::Break(_) => Err(FromTypstError::unexpected_node("break")),
            typst::ast::Expr::Continue(_) => Err(FromTypstError::unexpected_node("continue")),
            typst::ast::Expr::Return(_) => Err(FromTypstError::unexpected_node("return")),
            typst::ast::Expr::MathPrimes(_) => Err(FromTypstError::unexpected_node("math primes")),
            typst::ast::Expr::Contextual(_) => Err(FromTypstError::unexpected_node("contextual")),
        }
    }

    fn parbreak(&mut self, _node: &typst::ast::Parbreak) -> FromTypstResult<()> {
        self.push_paragraph_if_not_empty();
        Ok(())
    }

    fn heading(&mut self, node: &typst::ast::Heading, dic: &Dictionary) -> FromTypstResult<()> {
        let depth = node.depth().get();
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.export();
        self.push_segment(mathlog::Segment::Heading(mathlog::Heading {
            depth,
            content,
        }));
        self.push_paragraph();
        Ok(())
    }

    fn equation(&mut self, node: &typst::ast::Equation, dic: &Dictionary) -> FromTypstResult<()> {
        let mut writer = SegmentWriter::new();
        writer.math(&node.body(), dic)?;
        let content = writer.export();
        if node.block() {
            self.push_paragraph_if_not_empty();
            self.push_segment(mathlog::Segment::MathDisplay(mathlog::MathDisplay {
                content,
            }));
            self.push_paragraph();
        } else {
            self.push_segment(mathlog::Segment::MathInline(mathlog::MathInline {
                content,
            }));
        }
        Ok(())
    }

    fn list_item(&mut self, node: &typst::ast::ListItem, dic: &Dictionary) -> FromTypstResult<()> {
        self.push_paragraph_if_not_empty();
        let mut writer = ParagraphWriter::new();
        writer.markup(&node.body(), dic)?;
        self.push_segment(mathlog::Segment::ListItem(mathlog::ListItem {
            symbol: mathlog::ListSymbol::NoNum,
            contents: writer.export(),
        }));
        Ok(())
    }

    fn enum_item(&mut self, node: &typst::ast::EnumItem, dic: &Dictionary) -> FromTypstResult<()> {
        let symbol = mathlog::ListSymbol::NumDot(node.number().unwrap_or(1));
        self.push_paragraph_if_not_empty();
        let mut writer = ParagraphWriter::new();
        writer.markup(&node.body(), dic)?;
        self.push_segment(mathlog::Segment::ListItem(mathlog::ListItem {
            symbol,
            contents: writer.export(),
        }));
        Ok(())
    }

    fn show(&mut self, _node: &typst::ast::ShowRule) -> FromTypstResult<()> {
        // ignore but leave an export comment
        // self.push_paragraph_if_not_empty();
        self.segments_writer
            .push_segment(mathlog::Segment::ExportComment("#show".to_string()));
        Ok(())
    }

    fn func_call(&mut self, node: &typst::ast::FuncCall, dic: &Dictionary) -> FromTypstResult<()> {
        // TODO: add special cases, for example, for `#strong`
        match node.callee() {
            typst::ast::Expr::Ident(ident) => {
                let ident = ident.get();
                if let Some(kind) = mathlog::EnvKind::from_name(ident) {
                    self.env(node, dic, kind)
                } else {
                    todo!("unimplemented function: {:?}", node)
                }
            }
            _ => todo!("{:?}", node),
        }
    }

    fn import(&mut self, node: &typst::ast::ModuleImport) -> FromTypstResult<()> {
        // ignore but leave an export comment
        let mut import_str = "#import ".to_string();
        match node.source() {
            typst::ast::Expr::Ident(ident) => {
                import_str += ident.get();
            }
            typst::ast::Expr::Str(s) => {
                import_str += "\"";
                import_str += &s.get();
                import_str += "\"";
            }
            _ => {
                import_str += "?";
            }
        }
        if let Some(imports) = node.imports() {
            import_str += ": ";
            match imports {
                typst::ast::Imports::Wildcard => import_str += "*",
                typst::ast::Imports::Items(path) => {
                    import_str += &path
                        .iter()
                        .map(|item| item.bound_name().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            };
        }

        self.segments_writer
            .push_segment(mathlog::Segment::ExportComment(import_str));
        Ok(())
    }

    fn include(&mut self, node: &typst::ast::ModuleInclude) -> FromTypstResult<()> {
        // ignore but leave an export comment
        let mut include_str = "#include ".to_string();
        match node.source() {
            typst::ast::Expr::Ident(ident) => {
                include_str += ident.get();
            }
            typst::ast::Expr::Str(s) => {
                include_str += "\"";
                include_str += &s.get();
                include_str += "\"";
            }
            _ => {
                include_str += "?";
            }
        }

        self.segments_writer
            .push_segment(mathlog::Segment::ExportComment(include_str));
        Ok(())
    }

    fn link(&self, _link: &typst::ast::Link) -> Result<(), FromTypstError> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("link".to_string()))
    }

    fn label(&self, _label: &typst::ast::Label) -> Result<(), FromTypstError> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("label".to_string()))
    }

    fn ref_(&self, _ref_: &typst::ast::Ref) -> Result<(), FromTypstError> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("ref".to_string()))
    }

    fn term(&self, _term: &typst::ast::TermItem) -> Result<(), FromTypstError> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("term".to_string()))
    }

    //

    fn env(
        &mut self,
        node: &typst::ast::FuncCall,
        dic: &Dictionary,
        kind: mathlog::EnvKind,
    ) -> FromTypstResult<()> {
        let args = node.args();
        let mut title: Option<mathlog::Segments> = None;
        let mut body: Option<Vec<mathlog::Paragraph>> = None;
        for arg in args.items() {
            match arg {
                typst::ast::Arg::Pos(body_expr) => {
                    let mut writer = ParagraphWriter::new();
                    writer.eval_expr(&body_expr, dic)?;
                    let paragraphs = writer.export();
                    body = Some(paragraphs);
                }
                typst::ast::Arg::Named(named) => {
                    // removed: https://github.com/typst/typst/commit/be49935753f0e37ae8e04fb53111e6f116c63f47 between 0.10.0 and 0.11.0
                    //let Some(ident) = named.expr_ident() else {
                    //    todo!(); //TODO: handle error
                    //};
                    let ident = named.name();
                    match &*ident.get().to_string() {
                        "body" => {
                            let mut writer = ParagraphWriter::new();
                            writer.eval_expr(&named.expr(), dic)?;
                            body = Some(writer.export());
                        }
                        "title" => {
                            let mut writer = ParagraphWriter::new();
                            writer.eval_expr(&named.expr(), dic)?;
                            let paragraphs = writer.export();
                            if paragraphs.len() != 1 {
                                todo!(); //TODO: handle error
                            }
                            title = Some(paragraphs[0].segments.clone());
                        }
                        _ => todo!(), //TODO: handle error
                    }
                }
                typst::ast::Arg::Spread(_spread) => {
                    todo!() // TODO: handle error
                }
            }
        }
        let Some(contents) = body else {
            todo!(); // TODO: handle error
        };

        self.push_segment(mathlog::Segment::Env(mathlog::Env {
            kind,
            title,
            contents,
        }));
        self.push_paragraph();
        Ok(())
    }
}

impl SegmentWriter {
    fn markup(&mut self, node: &typst::ast::Markup, dic: &Dictionary) -> FromTypstResult<()> {
        for node in node.exprs() {
            self.expr(&node, dic)?;
        }
        Ok(())
    }

    fn expr(&mut self, node: &typst::ast::Expr, dic: &Dictionary) -> FromTypstResult<()> {
        match node {
            typst::ast::Expr::Linebreak(linebreak) => self.linebreak(linebreak),
            typst::ast::Expr::Text(text) => self.text(text),
            typst::ast::Expr::Escape(escape) => self.escape(escape),
            typst::ast::Expr::Str(str) => self.str(str),
            typst::ast::Expr::Raw(raw) => self.raw(raw),
            typst::ast::Expr::Space(space) => self.space(space),
            typst::ast::Expr::Equation(equation) => self.equation(equation, dic),
            typst::ast::Expr::Math(math) => self.math(math, dic),
            typst::ast::Expr::Ident(ident) => self.ident(ident, dic),
            typst::ast::Expr::MathIdent(ident) => self.math_ident(ident, dic),
            typst::ast::Expr::MathDelimited(delimited) => self.math_delimited(delimited, dic),
            typst::ast::Expr::MathAttach(attach) => self.math_attach(attach, dic),
            typst::ast::Expr::FieldAccess(access) => self.field_access(access, dic),
            typst::ast::Expr::FuncCall(call) => self.func_call(call, dic),
            typst::ast::Expr::Shorthand(shorthand) => self.shorthand(shorthand),
            typst::ast::Expr::MathAlignPoint(point) => self.math_align_point(point),
            typst::ast::Expr::MathFrac(frac) => self.math_frac(frac, dic),
            typst::ast::Expr::MathRoot(root) => self.math_root(root, dic),
            typst::ast::Expr::SmartQuote(quote) => self.smart_quote(quote),
            typst::ast::Expr::Strong(strong) => self.strong(strong, dic),
            typst::ast::Expr::Emph(emph) => self.emph(emph, dic),
            typst::ast::Expr::Link(link) => self.link(link),
            typst::ast::Expr::Label(label) => self.label(label),
            typst::ast::Expr::Ref(ref_) => self.ref_(ref_),
            //
            typst::ast::Expr::Parbreak(_) => Err(FromTypstError::unexpected_node("par")),
            typst::ast::Expr::Heading(_) => Err(FromTypstError::unexpected_node("heading")),
            typst::ast::Expr::List(_) => Err(FromTypstError::unexpected_node("list")),
            typst::ast::Expr::Enum(_) => Err(FromTypstError::unexpected_node("enum")),
            typst::ast::Expr::Term(_) => Err(FromTypstError::unexpected_node("term")),
            typst::ast::Expr::None(_) => Err(FromTypstError::unexpected_node("none")),
            typst::ast::Expr::Auto(_) => Err(FromTypstError::unexpected_node("auto")),
            typst::ast::Expr::Bool(_) => Err(FromTypstError::unexpected_node("bool")),
            typst::ast::Expr::Int(_) => Err(FromTypstError::unexpected_node("int")),
            typst::ast::Expr::Float(_) => Err(FromTypstError::unexpected_node("float")),
            typst::ast::Expr::Numeric(_) => Err(FromTypstError::unexpected_node("numeric")),
            typst::ast::Expr::Code(_) => Err(FromTypstError::unexpected_node("code")),
            typst::ast::Expr::Content(_) => Err(FromTypstError::unexpected_node("content")),
            typst::ast::Expr::Parenthesized(_) => {
                Err(FromTypstError::unexpected_node("parenthesized"))
            }
            typst::ast::Expr::Array(_) => Err(FromTypstError::unexpected_node("array")),
            typst::ast::Expr::Dict(_) => Err(FromTypstError::unexpected_node("dict")),
            typst::ast::Expr::Unary(_) => Err(FromTypstError::unexpected_node("unary")),
            typst::ast::Expr::Binary(_) => Err(FromTypstError::unexpected_node("binary")),
            typst::ast::Expr::Closure(_) => Err(FromTypstError::unexpected_node("closure")),
            typst::ast::Expr::Let(_) => Err(FromTypstError::unexpected_node("let")),
            typst::ast::Expr::DestructAssign(_) => {
                Err(FromTypstError::unexpected_node("destruct_assign"))
            }
            typst::ast::Expr::Set(_) => Err(FromTypstError::unexpected_node("set")),
            typst::ast::Expr::Show(_) => Err(FromTypstError::unexpected_node("show")),
            typst::ast::Expr::Conditional(_) => Err(FromTypstError::unexpected_node("conditional")),
            typst::ast::Expr::While(_) => Err(FromTypstError::unexpected_node("while")),
            typst::ast::Expr::For(_) => Err(FromTypstError::unexpected_node("for")),
            typst::ast::Expr::Import(_) => Err(FromTypstError::unexpected_node("import")),
            typst::ast::Expr::Include(_) => Err(FromTypstError::unexpected_node("include")),
            typst::ast::Expr::Break(_) => Err(FromTypstError::unexpected_node("break")),
            typst::ast::Expr::Continue(_) => Err(FromTypstError::unexpected_node("continue")),
            typst::ast::Expr::Return(_) => Err(FromTypstError::unexpected_node("return")),
            // _ => todo!("{:?}", node),
            typst::ast::Expr::MathPrimes(_) => Err(FromTypstError::unexpected_node("math primes")),
            typst::ast::Expr::Contextual(_) => Err(FromTypstError::unexpected_node("contextual")),
        }
    }

    fn linebreak(&mut self, _node: &typst::ast::Linebreak) -> FromTypstResult<()> {
        self.push_segment(mathlog::Segment::Linebreak);
        Ok(())
    }

    fn text(&mut self, node: &typst::ast::Text) -> FromTypstResult<()> {
        let text = node.get().to_string();
        let text = if text == "{" {
            "\\{".to_string()
        } else if text == "}" {
            "\\}".to_string()
        } else {
            text
        };
        self.push_segment(mathlog::Segment::Text(mathlog::Text(text)));
        Ok(())
    }

    fn escape(&mut self, node: &typst::ast::Escape) -> FromTypstResult<()> {
        let escaped = node.get();
        self.push_segment(mathlog::Segment::Text(mathlog::Text(escaped.to_string())));
        Ok(())
    }

    fn str(&mut self, node: &typst::ast::Str) -> FromTypstResult<()> {
        let s = node.get().to_string();
        self.push_segment(mathlog::Segment::Command(mathlog::Command {
            name: "mathrm".to_string(),
            args: vec![mathlog::Arg {
                is_optional: false,
                content: mathlog::Segments(vec![mathlog::Segment::Text(mathlog::Text(s))]),
            }],
        }));
        Ok(())
    }

    fn raw(&mut self, node: &typst::ast::Raw) -> FromTypstResult<()> {
        // removed: https://github.com/typst/typst/commit/030041466b5b8453ca23e43a6385f4592f78a56c between 0.10.0 and 0.11.0
        // let s = node.text().to_string();
        let s = node.lines()
            .map(|text| text.get().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        self.push_segment(mathlog::Segment::CodeInline(mathlog::CodeInline(s)));
        Ok(())
    }

    fn smart_quote(&mut self, node: &typst::ast::SmartQuote) -> FromTypstResult<()> {
        let q = if node.double() { '"' } else { '\'' };
        self.push_segment(mathlog::Segment::Text(mathlog::Text(q.to_string())));
        Ok(())
    }

    fn strong(&mut self, node: &typst::ast::Strong, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.export();
        self.push_segment(mathlog::Segment::Strong(mathlog::Strong { content }));
        Ok(())
    }

    fn emph(&mut self, node: &typst::ast::Emph, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.export();
        self.push_segment(mathlog::Segment::Emph(mathlog::Emph { content }));
        Ok(())
    }

    fn space(&mut self, _node: &typst::ast::Space) -> FromTypstResult<()> {
        // pass
        Ok(())
    }

    fn equation(&mut self, node: &typst::ast::Equation, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.math(&body, dic)?;
        let content = writer.export();
        self.push_segment(if node.block() {
            mathlog::Segment::MathDisplay(mathlog::MathDisplay { content })
        } else {
            mathlog::Segment::MathInline(mathlog::MathInline { content })
        });
        Ok(())
    }

    fn ident(&mut self, node: &typst::ast::Ident, dic: &Dictionary) -> FromTypstResult<()> {
        let ident = node.get().to_string();
        let command = dic
            .idents
            .get(&ident)
            .ok_or(FromTypstError::unsupported_ident(vec![ident]))?
            .clone();
        self.push_segment(mathlog::Segment::RawCommand(mathlog::RawCommand(command)));
        Ok(())
    }

    fn math_ident(
        &mut self,
        node: &typst::ast::MathIdent,
        dic: &Dictionary,
    ) -> FromTypstResult<()> {
        let ident = node.get().to_string();
        let command = dic
            .idents
            .get(&ident)
            .ok_or(FromTypstError::unsupported_ident(vec![ident]))?
            .clone();
        self.push_segment(mathlog::Segment::RawCommand(mathlog::RawCommand(command)));
        Ok(())
    }

    fn field_access(
        &mut self,
        node: &typst::ast::FieldAccess,
        dic: &Dictionary,
    ) -> FromTypstResult<()> {
        fn get_mod<'a>(
            node: &typst::ast::Expr,
            dic: &'a Dictionary,
        ) -> (Option<&'a Dictionary>, Vec<String>) {
            match node {
                typst::ast::Expr::FieldAccess(access) => {
                    let target = access.target();
                    let field = access.field();
                    let field_name = field.get().to_string();
                    let (dic, mut fields) = get_mod(&target, dic);
                    fields.push(field_name);
                    (dic, fields)
                }
                typst::ast::Expr::Ident(ident) => {
                    let ident = ident.get().to_string();
                    let dic = dic.modules.get(&ident);
                    (dic, vec![ident])
                }
                typst::ast::Expr::MathIdent(ident) => {
                    let ident = ident.get().to_string();
                    let dic = dic.modules.get(&ident);
                    (dic, vec![ident])
                }
                _ => panic!("unexpected node in field access: {:?}", node),
            }
        }

        let target = node.target();
        let field = node.field();
        let (mod_dic, path) = get_mod(&target, dic);
        let Some(mod_dic) = mod_dic else {
            return Err(FromTypstError::unsupported_ident(path));
        };
        let field_name = field.get().to_string();
        let command = mod_dic
            .idents
            .get(&field_name)
            .ok_or(FromTypstError::unsupported_ident(path))?
            .clone();
        self.push_segment(mathlog::Segment::RawCommand(mathlog::RawCommand(command)));
        Ok(())
    }

    fn func_call(&mut self, node: &typst::ast::FuncCall, dic: &Dictionary) -> FromTypstResult<()> {
        // TODO: add special cases, for example, for `#strong`
        match node.callee() {
            typst::ast::Expr::Ident(ident) => {
                let ident = ident.get();
                if let Some(kind) = mathlog::EnvKind::from_name(ident) {
                    Err(FromTypstError::env_in_segments(kind.name()))
                } else {
                    todo!(); // TODO: handle other cases
                }
            }
            typst::ast::Expr::MathIdent(ident) => {
                let ident = ident.get();
                match &*ident.to_string() {
                    "upright" => self.single_call("mathrm", &node.args(), dic),
                    "italic" => self.single_call("mathit", &node.args(), dic),
                    "bold" => self.single_call("boldsymbol", &node.args(), dic),
                    "cal" => self.single_call("mathcal", &node.args(), dic),
                    "bb" => self.single_call("mathbb", &node.args(), dic),
                    "frak" => self.single_call("mathfrak", &node.args(), dic),
                    _ => todo!("{:?}", node),
                }
            }
            _ => todo!("{:?}", node),
        }
    }

    fn shorthand(&mut self, node: &typst::ast::Shorthand) -> FromTypstResult<()> {
        let shorthand = node.get();
        let shorthand = if shorthand == '′' { '\'' } else { shorthand };
        self.push_segment(mathlog::Segment::Text(mathlog::Text(shorthand.to_string())));
        Ok(())
    }

    fn math_delimited(
        &mut self,
        node: &typst::ast::MathDelimited,
        dic: &Dictionary,
    ) -> FromTypstResult<()> {
        let open = node.open();
        let body = node.body();
        let close = node.close();

        let mut open_writer = SegmentWriter::new();
        open_writer.expr(&open, dic)?;
        let open_content = open_writer.export();

        let mut body_writer = SegmentWriter::new();
        body_writer.math(&body, dic)?;
        let body_content = body_writer.export();

        let mut close_writer = SegmentWriter::new();
        close_writer.expr(&close, dic)?;
        let close_content = close_writer.export();

        self.push_segment(mathlog::Segment::MathDelimited(mathlog::MathDelimited {
            open: open_content,
            body: body_content,
            close: close_content,
        }));

        Ok(())
    }

    fn math_attach(
        &mut self,
        node: &typst::ast::MathAttach,
        dic: &Dictionary,
    ) -> FromTypstResult<()> {
        let base = node.base();
        let bottom = node.bottom();
        let top = node.top();

        let mut base_writer = SegmentWriter::new();
        base_writer.expr(&base, dic)?;
        let base_content = base_writer.export();

        let bottom_content = if let Some(bottom) = bottom {
            let mut bottom_writer = SegmentWriter::new();
            bottom_writer.expr(&bottom, dic)?;
            Some(bottom_writer.export())
        } else {
            None
        };

        let top_content = if let Some(top) = top {
            let mut top_writer = SegmentWriter::new();
            top_writer.expr(&top, dic)?;
            Some(top_writer.export())
        } else {
            None
        };

        self.push_segment(mathlog::Segment::MathAttach(mathlog::MathAttach {
            base: base_content,
            bottom: bottom_content,
            top: top_content,
        }));

        Ok(())
    }

    fn math_align_point(&mut self, _node: &typst::ast::MathAlignPoint) -> FromTypstResult<()> {
        self.push_segment(mathlog::Segment::MathAlignPoint);
        Ok(())
    }

    fn math_frac(&mut self, node: &typst::ast::MathFrac, dic: &Dictionary) -> FromTypstResult<()> {
        let num = node.num();
        let denom = node.denom();

        let mut num_writer = SegmentWriter::new();
        num_writer.expr(&num, dic)?;
        let num_content = num_writer.export();

        let mut denom_writer = SegmentWriter::new();
        denom_writer.expr(&denom, dic)?;
        let denom_content = denom_writer.export();

        self.push_segment(mathlog::Segment::Command(mathlog::Command {
            name: "frac".to_string(),
            args: vec![
                mathlog::Arg {
                    is_optional: false,
                    content: num_content,
                },
                mathlog::Arg {
                    is_optional: false,
                    content: denom_content,
                },
            ],
        }));

        Ok(())
    }

    fn math_root(&mut self, node: &typst::ast::MathRoot, dic: &Dictionary) -> FromTypstResult<()> {
        let index = node.index();
        let radicand = node.radicand();

        let mut radicand_writer = SegmentWriter::new();
        radicand_writer.expr(&radicand, dic)?;
        let radicand_content = radicand_writer.export();

        let arg = if let Some(index) = index {
            vec![
                mathlog::Arg {
                    is_optional: true,
                    content: mathlog::Segments(vec![mathlog::Segment::Text(mathlog::Text(
                        index.to_string(),
                    ))]),
                },
                mathlog::Arg {
                    is_optional: false,
                    content: radicand_content,
                },
            ]
        } else {
            vec![mathlog::Arg {
                is_optional: false,
                content: radicand_content,
            }]
        };
        self.push_segment(mathlog::Segment::Command(mathlog::Command {
            name: "sqrt".to_string(),
            args: arg,
        }));
        Ok(())
    }

    fn link(&self, _link: &typst::ast::Link) -> FromTypstResult<()> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("link".to_string()))
    }

    fn label(&self, _label: &typst::ast::Label) -> FromTypstResult<()> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("label".to_string()))
    }

    fn ref_(&self, _ref_: &typst::ast::Ref) -> FromTypstResult<()> {
        // TODO: implement
        Err(FromTypstError::not_yet_implemented("ref".to_string()))
    }

    //

    fn math(&mut self, node: &typst::ast::Math, dic: &Dictionary) -> FromTypstResult<()> {
        for expr in node.exprs() {
            self.expr(&expr, dic)?;
        }
        Ok(())
    }

    fn single_call(
        &mut self,
        name: &str,
        args: &typst::ast::Args,
        dic: &Dictionary,
    ) -> FromTypstResult<()> {
        if args.items().count() != 1 {
            todo!(); // TODO: handle error
        }
        let arg = args.items().next().unwrap();
        let mut body = None; // TODO
        match arg {
            typst::ast::Arg::Pos(arg) => {
                body = Some(arg);
            }
            typst::ast::Arg::Named(arg) => {
                body = Some(arg.expr());
            }
            _ => todo!(), // TODO: handle error
        }
        let Some(body) = body else {
            todo!(); // TODO: handle error
        };

        let mut writer = SegmentWriter::new();
        writer.expr(&body, dic)?;
        let content = writer.export();
        self.push_segment(mathlog::Segment::Command(mathlog::Command {
            name: name.to_string(),
            args: vec![mathlog::Arg {
                is_optional: false,
                content,
            }],
        }));
        Ok(())
    }
}

impl ParagraphWriter {
    fn eval_expr(&mut self, node: &typst::ast::Expr, dic: &Dictionary) -> FromTypstResult<()> {
        match node {
            typst::ast::Expr::Str(s) => {
                let s = s.get().to_string();
                self.push_segment(mathlog::Segment::Text(mathlog::Text(s)));
                Ok(())
            }
            typst::ast::Expr::Content(content_block) => {
                let markup = content_block.body();
                self.markup(&markup, dic)
            }
            _ => todo!("{:?}", node),
        }
    }
}
