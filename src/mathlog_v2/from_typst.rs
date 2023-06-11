use super::ast as mathlog;
use super::*;
use typst::syntax as typst;

use core::fmt;

//

#[derive(Debug)]
pub enum FromTypstErrorKind {
    UnsupportedIdent(Vec<String>),
    UnsupportedModule(Vec<String>),
}

impl fmt::Display for FromTypstErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromTypstErrorKind::UnsupportedIdent(s) => {
                write!(f, "UnsupportedIdent: {}", s.join("."))
            }
            FromTypstErrorKind::UnsupportedModule(s) => {
                write!(f, "UnsupportedModule: {}", s.join("."))
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

impl FromTypstError {
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
}

pub type FromTypstResult<T> = Result<T, FromTypstError>;

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

    fn return_segments(self) -> mathlog::Segments {
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
        let segments = std::mem::take(&mut self.segments_writer).return_segments();
        self.paragraphs.push(mathlog::Paragraph { segments });
    }

    fn push_paragraph_if_not_empty(&mut self) {
        if !self.segments_writer.is_empty() {
            self.push_paragraph();
        }
    }

    fn return_paragraphs(mut self) -> Vec<mathlog::Paragraph> {
        self.push_paragraph_if_not_empty();
        self.paragraphs
    }
}

//

impl mathlog::Syntax {
    pub fn from_typst(node: &typst::ast::Markup, dic: &Dictionary) -> FromTypstResult<Self> {
        let mut writer = ParagraphWriter::new();
        writer.markup(node, dic)?;
        let paragraphs = writer.return_paragraphs();
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
            _ => todo!("{:?}", node),
        }
    }

    fn parbreak(&mut self, _node: &typst::ast::Parbreak) -> FromTypstResult<()> {
        self.push_paragraph_if_not_empty();
        Ok(())
    }

    fn heading(&mut self, node: &typst::ast::Heading, dic: &Dictionary) -> FromTypstResult<()> {
        let level = node.level().get();
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.return_segments();
        self.push_segment(mathlog::Segment::Heading(mathlog::Heading {
            level,
            content,
        }));
        self.push_paragraph();
        Ok(())
    }

    fn equation(&mut self, node: &typst::ast::Equation, dic: &Dictionary) -> FromTypstResult<()> {
        self.push_paragraph_if_not_empty();
        let mut writer = SegmentWriter::new();
        writer.math(&node.body(), dic)?;
        let content = writer.return_segments();
        self.push_segment(mathlog::Segment::MathDisplay(mathlog::MathDisplay {
            content,
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
            typst::ast::Expr::Text(text) => self.text(text),
            typst::ast::Expr::Space(space) => self.space(space),
            typst::ast::Expr::Equation(equation) => self.equation(equation, dic),
            typst::ast::Expr::Math(math) => self.math(math, dic),
            typst::ast::Expr::Ident(ident) => self.ident(ident, dic),
            typst::ast::Expr::MathIdent(ident) => self.math_ident(ident, dic),
            typst::ast::Expr::MathDelimited(delimited) => self.math_delimited(delimited, dic),
            typst::ast::Expr::MathAttach(attach) => self.math_attach(attach, dic),
            typst::ast::Expr::FieldAccess(access) => self.field_access(access, dic),
            _ => todo!("{:?}", node),
        }
    }

    fn text(&mut self, node: &typst::ast::Text) -> FromTypstResult<()> {
        let text = node.get().to_string();
        self.push_segment(mathlog::Segment::Text(mathlog::Text(text)));
        Ok(())
    }

    fn strong(&mut self, node: &typst::ast::Strong, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.return_segments();
        self.push_segment(mathlog::Segment::Strong(mathlog::Strong { content }));
        Ok(())
    }

    fn emph(&mut self, node: &typst::ast::Emph, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.markup(&body, dic)?;
        let content = writer.return_segments();
        self.push_segment(mathlog::Segment::Emph(mathlog::Emph { content }));
        Ok(())
    }

    fn space(&mut self, _node: &typst::ast::Space) -> FromTypstResult<()> {
        // self.push_segment(mathlog::Segment::Space);
        Ok(())
    }

    fn equation(&mut self, node: &typst::ast::Equation, dic: &Dictionary) -> FromTypstResult<()> {
        let body = node.body();
        let mut writer = SegmentWriter::new();
        writer.math(&body, dic)?;
        let content = writer.return_segments();
        self.push_segment(mathlog::Segment::MathInline(mathlog::MathInline {
            content,
        }));
        Ok(())
    }

    fn ident(&mut self, node: &typst::ast::Ident, dic: &Dictionary) -> FromTypstResult<()> {
        let ident = node.get().to_string();
        let command = dic
            .idents
            .get(&ident)
            .ok_or(FromTypstError::unsupported_ident(vec![ident]))?
            .clone();
        self.push_segment(mathlog::Segment::Command(mathlog::Command(command)));
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
        self.push_segment(mathlog::Segment::Command(mathlog::Command(command)));
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
        self.push_segment(mathlog::Segment::Command(mathlog::Command(command)));
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
        let open_content = open_writer.return_segments();

        let mut body_writer = SegmentWriter::new();
        body_writer.math(&body, dic)?;
        let body_content = body_writer.return_segments();

        let mut close_writer = SegmentWriter::new();
        close_writer.expr(&close, dic)?;
        let close_content = close_writer.return_segments();

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
        let base_content = base_writer.return_segments();

        let bottom_content = if let Some(bottom) = bottom {
            let mut bottom_writer = SegmentWriter::new();
            bottom_writer.expr(&bottom, dic)?;
            Some(bottom_writer.return_segments())
        } else {
            None
        };

        let top_content = if let Some(top) = top {
            let mut top_writer = SegmentWriter::new();
            top_writer.expr(&top, dic)?;
            Some(top_writer.return_segments())
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

    //

    fn math(&mut self, node: &typst::ast::Math, dic: &Dictionary) -> FromTypstResult<()> {
        for expr in node.exprs() {
            self.expr(&expr, dic)?;
        }
        Ok(())
    }
}
