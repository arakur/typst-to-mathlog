use super::ast::*;
use crate::utils::{add_indent, Intercalate};

//

const TAB_SPACE_NUM: usize = 4;

//

//

impl Syntax {
    pub fn compose(&self) -> String {
        self.paragraphs.compose()
    }
}

impl Paragraphs {
    fn compose(&self) -> String {
        self.0.iter().map(|p| p.compose()).intercalate("\n\n")
    }
}

impl Paragraph {
    fn compose(&self) -> String {
        self.lines.iter().map(|l| l.compose()).intercalate("\n")
    }
}

impl Line {
    fn compose(&self) -> String {
        match self {
            Line::Heading(h) => h.compose(),
            Line::Text(t) => t.compose(),
            Line::Env(e) => e.compose(),
            Line::Enum(e) => e.compose(),
            Line::Quote(q) => q.compose(),
            Line::Code(c) => c.compose(),
        }
    }
}

impl Heading {
    fn compose(&self) -> String {
        format!(
            "{} {}",
            "#".repeat(self.level),
            self.segments.0.iter().map(|s| s.compose()).intercalate(" ")
        )
    }
}

impl Text {
    fn compose(&self) -> String {
        self.segments.0.iter().map(|s| s.compose()).intercalate("")
    }
}

impl Segment {
    fn compose(&self) -> String {
        match self {
            Segment::Plain(s) => s.clone(),
            Segment::MathInline(m) => format!("${}$", m.compose()),
            Segment::MathDisplay(m) => format!("$$\n{}\n$$", m.compose()),
            Segment::HTMLTag(t) => t.compose(),
            Segment::Emph(e) => e.compose(),
            Segment::Link(l) => l.compose(),
            Segment::Table(t) => t.compose(),
            Segment::Image(i) => i.compose(),
        }
    }
}

impl Math {
    fn compose(&self) -> String {
        self.segments.0.iter().map(|s| s.compose()).intercalate(" ")
    }
}

impl MathSegment {
    fn compose(&self) -> String {
        match self {
            MathSegment::Single(s) => s.clone(),
            MathSegment::Braced { segments } => format!(
                "{{{}}}",
                segments.0.iter().map(|s| s.compose()).intercalate(" ")
            ),
            MathSegment::Delimited {
                left,
                segments,
                right,
            } => format!(
                "\\left{} {} \\right{}",
                left,
                segments.0.iter().map(|s| s.compose()).intercalate(" "),
                right
            ),
            MathSegment::Function { name, arguments } => format!(
                "\\{}{}",
                name,
                arguments.iter().map(|a| a.compose()).intercalate("")
            ),
        }
    }
}

impl MathArg {
    fn compose(&self) -> String {
        match self {
            MathArg::Optional(m) => format!("[{}]", m.compose()),
            MathArg::Required(m) => format!("{{{}}}", m.compose()),
        }
    }
}

impl HTMLTag {
    fn compose(&self) -> String {
        format!(
            "<{}{}>{}</{}>",
            self.name,
            self.attributes
                .iter()
                .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                .intercalate(""),
            self.children.compose(),
            self.name
        )
    }
}

impl Emph {
    fn compose(&self) -> String {
        let tag = match self.kind {
            EmphKind::Bold => "b",
            EmphKind::Italic => "i",
            EmphKind::Strike => "s",
        };
        format!("<{}>{}</{}>", tag, self.child.compose(), tag)
    }
}

impl Link {
    fn compose(&self) -> String {
        format!("[{}]({})", self.text, self.url)
    }
}

impl Table {
    fn compose(&self) -> String {
        format!(
            "{}\n{}",
            self.header.compose(),
            self.rows.iter().map(|r| r.compose()).intercalate("\n")
        )
    }
}

impl TableRow {
    fn compose(&self) -> String {
        format!(
            "|{}|",
            self.cells
                .iter()
                .map(|segments| segments.0.iter().map(|s| s.compose()).intercalate(" "))
                .intercalate("|")
        )
    }
}

impl Image {
    fn compose(&self) -> String {
        format!("![]({})", self.url)
    }
}

impl Env {
    fn compose(&self) -> String {
        let env_name = self.kind.name();
        let env_title = self
            .title
            .as_ref()
            .map(|ss| ss.0.iter().map(|s| s.compose()).intercalate(" "))
            .unwrap_or_else(String::new);
        let env_body = self.children.compose();
        format!("&&&{} {}\n{}\n&&&", env_name, env_title, env_body)
    }
}

impl EnvKind {
    fn name(&self) -> &str {
        match self {
            EnvKind::Block => "",
            EnvKind::Conj => "conj",
            EnvKind::Axm => "axm",
            EnvKind::Def => "def",
            EnvKind::Prop => "prop",
            EnvKind::Fml => "fml",
            EnvKind::Lem => "lem",
            EnvKind::Thm => "thm",
            EnvKind::Cor => "cor",
            EnvKind::Prf => "prf",
            EnvKind::Ex => "ex",
            EnvKind::Exc => "exc",
            EnvKind::Rem => "rem",
        }
    }
}

impl Enum {
    fn compose(&self) -> String {
        // indent all inner lines
        self.children
            .iter()
            .enumerate()
            .map(|(i, ps)| {
                let numbering = self.kind.numbering(i);
                let numbering_len = numbering.chars().count();
                let space_num = TAB_SPACE_NUM * (numbering_len / TAB_SPACE_NUM + 1);
                let init_spacing = space_num - numbering_len;
                let rest_spacing = space_num;
                // spacing TAB_SPACE_NUM - numbering_len for the first line
                // and TAB_SPACE_NUM for the rest
                numbering + &add_indent(&ps.compose(), init_spacing, rest_spacing)
            })
            .intercalate("\n")
    }
}

impl EnumKind {
    fn numbering(&self, i: usize) -> String {
        match self {
            EnumKind::NoNum => "-".to_string(),
            EnumKind::Num { num_kind, style } => {
                let num = match num_kind {
                    NumKind::Arabic => format!("{}", i + 1),
                    NumKind::Roman => format!("R{}", i + 1),
                };
                match style {
                    EnumNumStyle::Period => format!("{}.", num),
                    EnumNumStyle::Paren => format!("({})", num),
                    EnumNumStyle::Square => format!("[{}]", num),
                }
            }
        }
    }
}

impl Quote {
    fn compose(&self) -> String {
        self.children
            .compose()
            .lines()
            .map(|l| format!("> {}", l))
            .intercalate("\n")
    }
}

impl CodeBlock {
    fn compose(&self) -> String {
        let lang = self.lang.as_deref().unwrap_or("");
        format!(
            "```{}\n{}\n```",
            lang,
            add_indent(&self.code, TAB_SPACE_NUM, TAB_SPACE_NUM)
        )
    }
}

// tests

// TODO: make tests for all variants

#[test]
fn _compose_sample00() {
    let input = Syntax {
        paragraphs: Paragraphs(vec![
            Paragraph {
                lines: vec![Line::Heading(Heading {
                    level: 1,
                    segments: Segments(vec![Segment::Plain("Greeting".to_string())]),
                })],
            },
            Paragraph {
                lines: vec![Line::Heading(Heading {
                    level: 2,
                    segments: Segments(vec![Segment::Plain("Hello".to_string())]),
                })],
            },
            Paragraph {
                lines: vec![Line::Text(Text {
                    segments: Segments(vec![
                        Segment::Plain("Hello! ".to_string()),
                        Segment::MathInline(Math {
                            segments: MathSegments(vec![
                                MathSegment::Single("a".to_string()),
                                MathSegment::Single("+".to_string()),
                                MathSegment::Delimited {
                                    left: "(".to_string(),
                                    segments: MathSegments(vec![
                                        MathSegment::Single("b".to_string()),
                                        MathSegment::Single("\\times".to_string()),
                                        MathSegment::Function {
                                            name: "frac".to_string(),
                                            arguments: vec![
                                                MathArg::Required(Math {
                                                    segments: MathSegments(vec![
                                                        MathSegment::Single("c".to_string()),
                                                    ]),
                                                }),
                                                MathArg::Required(Math {
                                                    segments: MathSegments(vec![
                                                        MathSegment::Single("d".to_string()),
                                                    ]),
                                                }),
                                            ],
                                        },
                                    ]),
                                    right: ")".to_string(),
                                },
                            ]),
                        }),
                    ]),
                })],
            },
            Paragraph {
                lines: vec![Line::Text(Text {
                    segments: Segments(vec![Segment::Plain(
                        "Here is a table of contents:".to_string(),
                    )]),
                })],
            },
            Paragraph {
                lines: vec![Line::Enum(Enum {
                    kind: EnumKind::NoNum,
                    children: vec![
                        Paragraph {
                            lines: vec![
                                Line::Text(Text {
                                    segments: Segments(vec![Segment::Plain(
                                        "Greeting".to_string(),
                                    )]),
                                }),
                                Line::Enum(Enum {
                                    kind: EnumKind::Num {
                                        num_kind: NumKind::Arabic,
                                        style: EnumNumStyle::Period,
                                    },
                                    children: vec![Paragraph {
                                        lines: vec![Line::Text(Text {
                                            segments: Segments(vec![Segment::Plain(
                                                "Hello".to_string(),
                                            )]),
                                        })],
                                    }],
                                }),
                            ],
                        },
                        Paragraph {
                            lines: vec![
                                Line::Text(Text {
                                    segments: Segments(vec![Segment::Plain("Bye".to_string())]),
                                }),
                                Line::Enum(Enum {
                                    kind: EnumKind::Num {
                                        num_kind: NumKind::Roman,
                                        style: EnumNumStyle::Paren,
                                    },
                                    children: vec![
                                        Paragraph {
                                            lines: vec![Line::Text(Text {
                                                segments: Segments(vec![Segment::Plain(
                                                    "Goodbye".to_string(),
                                                )]),
                                            })],
                                        },
                                        Paragraph {
                                            lines: vec![Line::Text(Text {
                                                segments: Segments(vec![Segment::Plain(
                                                    "See you".to_string(),
                                                )]),
                                            })],
                                        },
                                    ],
                                }),
                            ],
                        },
                    ],
                })],
            },
        ]),
    };
    let output = r#"
# Greeting

## Hello

Hello! $a + \left(b \times \frac{c}{d}\right)$

Here is a table of contents:

-   Greeting
    1.  Hello
-   Bye
    (R1)    Goodbye
    (R2)    See you
"#
    .trim();
    assert_eq!(input.compose(), output);
}
