use std::collections::HashMap;

use super::ast::{self as mathlog};
use typst::syntax as typst;

//

#[derive(Debug)]
pub struct ToMathlogError {
    kind: ToMathlogErrorKind,
}

#[derive(Debug)]
pub enum ToMathlogErrorKind {
    UnsupportedMathIdent(String),
    UnsupportedModule(String),
}

type ToMathlogResult<T> = Result<T, ToMathlogError>;

//

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Dictionary {
    pub idents: HashMap<String, String>,
    pub modules: HashMap<String, Dictionary>,
}

impl Dictionary {
    pub fn new() -> Self {
        Self {
            idents: HashMap::new(),
            modules: HashMap::new(),
        }
    }

    pub fn insert_ident(&mut self, ident: &str, math_ident: &str) {
        self.idents
            .insert(ident.to_string(), math_ident.to_string());
    }

    pub fn insert_mod(&mut self, mod_name: &str, dic: Dictionary) {
        self.modules.insert(mod_name.to_string(), dic);
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}
fn lookup_ident(s: &str, dic: &Dictionary) -> ToMathlogResult<mathlog::MathSegment> {
    let name = dic.idents.get(s).ok_or_else(|| ToMathlogError {
        kind: ToMathlogErrorKind::UnsupportedMathIdent(s.to_string()),
    })?;
    Ok(mathlog::MathSegment::Single(name.to_string()))
}

//

impl mathlog::Syntax {
    pub fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Self> {
        assert_eq!(
            node.kind(),
            typst::SyntaxKind::Markup,
            "a markup node expected"
        );
        let mut paragraphs = mathlog::Paragraphs::new();
        mathlog::Syntax::write(&mut paragraphs, node, dic)?;
        Ok(mathlog::Syntax { paragraphs })
    }

    fn write(
        paragraphs: &mut mathlog::Paragraphs,
        node: &typst::SyntaxNode,
        dic: &Dictionary,
    ) -> ToMathlogResult<()> {
        assert_eq!(
            node.kind(),
            typst::SyntaxKind::Markup,
            "a markup node expected"
        );
        let mut paragraph = mathlog::Paragraph::new();
        todo!()
    }
}

// impl mathlog::Syntax {
//     pub fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Self> {
//         assert_eq!(node.kind(), typst::SyntaxKind::Markup);
//         let paragraphs = collect_lines(node, dic)?;
//         Ok(mathlog::Syntax { paragraphs })
//     }
// }

// fn collect_lines(
//     node: &typst::SyntaxNode,
//     dic: &Dictionary,
// ) -> ToMathlogResult<mathlog::Paragraphs> {
//     assert_eq!(node.kind(), typst::SyntaxKind::Markup);
//     let mut paragraphs = Vec::new();
//     let mut lines = Vec::new();
//     for child in node.children() {
//         match child.kind() {
//             typst::SyntaxKind::Heading => {
//                 let line = to_heading(child, dic)?;
//                 lines.push(line);
//                 paragraphs.push(mathlog::Paragraph { lines });
//                 lines = Vec::new();
//             }
//             typst::SyntaxKind::Space => {
//                 // pass
//             }
//             typst::SyntaxKind::Text => {
//                 let text = child.text().to_string();
//                 let segments = mathlog::Segments(vec![mathlog::Segment::Plain(text)]);
//                 let line = mathlog::Line::Text(mathlog::Text { segments });
//                 lines.push(line);
//             }
//             typst::SyntaxKind::Strong => {
//                 let (_star0, body, _star1) = (
//                     child.children().next().expect("*"),
//                     child.children().nth(1).expect("body"),
//                     child.children().nth(2).expect("*"),
//                 );
//                 let paragraphs = collect_lines(body, dic)?;
//                 assert_eq!(paragraphs.0.len(), 1);
//                 let paragraph = paragraphs.0.into_iter().next().unwrap();
//                 let strong = mathlog::Emph {
//                     kind: mathlog::EmphKind::Bold,
//                     child: paragraph,
//                 };
//                 let segments = mathlog::Segments(vec![mathlog::Segment::Emph(strong)]);
//                 let line = mathlog::Line::Text(mathlog::Text { segments });
//                 lines.push(line);
//             }
//             _ => todo!("for {:?}", child.kind()),
//         }
//     }
//     if !lines.is_empty() {
//         paragraphs.push(mathlog::Paragraph { lines });
//     }
//     Ok(mathlog::Paragraphs(paragraphs))
// }

// fn to_heading(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<mathlog::Line> {
//     assert_eq!(node.kind(), typst::SyntaxKind::Heading);
//     let mut children = node.children();
//     let (heading_marker, _space, body_node) = (
//         children.next().expect("heading marker"),
//         children.next(),
//         children.next(),
//     );
//     let level = heading_marker.text().chars().count();
//     let segments = body_node
//         .map(|seg| mathlog::Segments::from_typst(seg, dic))
//         .unwrap_or(Ok(mathlog::Segments(Vec::new())))?;

//     Ok(mathlog::Line::Heading(mathlog::Heading { level, segments }))
// }

// impl mathlog::Heading {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Text {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Segments {
//     fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Self> {
//         assert_eq!(node.kind(), typst::SyntaxKind::Markup);
//         let segments = node
//             .children()
//             .map(|seg| mathlog::Segment::from_typst(seg, dic))
//             .collect::<ToMathlogResult<Vec<Option<_>>>>()?
//             .into_iter()
//             .flatten()
//             .collect();
//         Ok(mathlog::Segments(segments))
//     }
// }

// impl mathlog::Segment {
//     // None if it is a space
//     fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Option<Self>> {
//         match node.kind() {
//             typst::SyntaxKind::Text => {
//                 let text = node.text().to_string();
//                 Ok(Some(mathlog::Segment::Plain(text)))
//             }
//             typst::SyntaxKind::Space => {
//                 // skipped
//                 Ok(None)
//             }
//             typst::SyntaxKind::Equation => {
//                 let mut children = node.children();
//                 let (_dollar0, body, _dollar1) = (
//                     children.next().expect("$"),
//                     children.next().expect("body"),
//                     children.next().expect("$"),
//                 );

//                 let math = mathlog::Math::from_typst(body, dic)?;
//                 Ok(Some(mathlog::Segment::MathInline(math)))
//             }
//             _ => todo!("for typst::SyntaxKind::{:#?}", node.kind()),
//         }
//     }
// }

// impl mathlog::Math {
//     fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Self> {
//         assert_eq!(node.kind(), typst::SyntaxKind::Math);
//         Ok(mathlog::Math {
//             segments: mathlog::MathSegments::from_typst(node, dic)?,
//         })
//     }
// }

// impl mathlog::MathSegments {
//     fn from_typst(node: &typst::SyntaxNode, dic: &Dictionary) -> ToMathlogResult<Self> {
//         let mut segments = mathlog::MathSegments(Vec::new());
//         match node.kind() {
//             typst::SyntaxKind::Math => {
//                 for child in node.children() {
//                     add_node_to_segment(&mut segments, child, dic)?;
//                 }
//             }
//             _ => {
//                 add_node_to_segment(&mut segments, node, dic)?;
//             }
//         }
//         Ok(segments)
//     }
// }

// fn add_node_to_segment(
//     segments: &mut mathlog::MathSegments,
//     node: &typst::SyntaxNode,
//     dic: &Dictionary,
// ) -> ToMathlogResult<()> {
//     match node.kind() {
//         typst::SyntaxKind::Text => {
//             let text = node.text().to_string();
//             segments.0.push(mathlog::MathSegment::Single(text));
//         }
//         typst::SyntaxKind::MathIdent => {
//             let ident = lookup_ident(node.text(), dic)?;
//             segments.0.push(ident);
//         }
//         typst::SyntaxKind::MathAttach => {
//             // a _ 1 ^ 2
//             // push a central node
//             let mut attach_children = node.children();
//             let central_node = attach_children.next().expect("central");
//             let central = mathlog::MathSegments::from_typst(central_node, dic)?;

//             // push central if it is a single node
//             // otherwise wrap it with {}
//             if central.0.len() == 1 {
//                 // pop the single node and push
//                 let mut central = central;
//                 let elem = central.0.pop().unwrap();
//                 segments.0.push(elem);
//             } else {
//                 let central_braced = mathlog::MathSegment::Braced { segments: central };
//                 segments.0.push(central_braced);
//             }
//             // push attached nodes
//             while let Some(attach_symbol) = attach_children.next() {
//                 let attached_node = attach_children.next().expect("attached term");
//                 let attach_symbol = attach_symbol.text().to_string();
//                 assert!(attach_symbol == "_" || attach_symbol == "^");
//                 // push the attach symbol
//                 segments.0.push(mathlog::MathSegment::Single(attach_symbol));
//                 // push the attached node
//                 // if the attached node have `(` and `)` around it, remove them
//                 let mut attached_node_children = attached_node.children();
//                 let attached_node_children_wo_paren =
//                     if let (Some(paren0), Some(node), Some(paren1)) = (
//                         attached_node_children.next(),
//                         attached_node_children.next(),
//                         attached_node_children.next(),
//                     ) {
//                         assert_eq!(paren0.kind(), typst::SyntaxKind::LeftParen);
//                         assert_eq!(paren1.kind(), typst::SyntaxKind::RightParen);
//                         node
//                     } else {
//                         attached_node
//                     };
//                 let attached =
//                     mathlog::MathSegments::from_typst(attached_node_children_wo_paren, dic)?;
//                 if attached.0.len() == 1 {
//                     // pop the single node and push
//                     let mut attached = attached;
//                     let elem = attached.0.pop().unwrap();
//                     segments.0.push(elem);
//                 } else {
//                     let attached_braced = mathlog::MathSegment::Braced { segments: attached };
//                     segments.0.push(attached_braced);
//                 }
//             }
//         }
//         typst::SyntaxKind::MathDelimited => {
//             // [ body ]
//             let mut children = node.children();
//             let (left, body, right) = (
//                 children.next().expect("left"),
//                 children.next().expect("body"),
//                 children.next().expect("right"),
//             );
//             let s = mathlog::MathSegments::from_typst(body, dic)?;
//             let delimited = mathlog::MathSegment::Delimited {
//                 left: left.text().to_string(),
//                 segments: s,
//                 right: right.text().to_string(),
//             };
//             segments.0.push(delimited);
//         }
//         typst::SyntaxKind::Shorthand => {
//             let shorthand = node.text().to_string();
//             segments.0.push(mathlog::MathSegment::Single(shorthand));
//         }
//         typst::SyntaxKind::FieldAccess => {
//             access_field(segments, node, dic)?;
//         }
//         typst::SyntaxKind::Space => {
//             // pass
//         }
//         _ => todo!("for typst::SyntaxKind::{:#?}", node.kind()),
//     }
//     Ok(())
// }

// fn access_field(
//     segments: &mut mathlog::MathSegments,
//     node: &typst::SyntaxNode,
//     dic: &Dictionary,
// ) -> ToMathlogResult<()> {
//     let mut children = node.children();
//     let (left, dot, right) = (
//         children.next().expect("left"),
//         children.next().expect("dot"),
//         children.next().expect("right"),
//     );
//     assert_eq!(dot.text(), ".");
//     let dic0 = access_module(left, dic)?;
//     let right_name = right.text().to_string();
//     let latex = lookup_ident(&right_name, dic0)?;
//     segments.0.push(latex);
//     Ok(())
// }

// fn access_module<'a>(
//     node: &typst::SyntaxNode,
//     dic: &'a Dictionary,
// ) -> ToMathlogResult<&'a Dictionary> {
//     match node.kind() {
//         typst::SyntaxKind::MathIdent => {
//             let left_name = node.text().to_string();
//             let dic0 = dic.modules.get(&left_name).ok_or(ToMathlogError {
//                 kind: ToMathlogErrorKind::UnsupportedModule(left_name),
//             })?;
//             Ok(dic0)
//         }
//         typst::SyntaxKind::FieldAccess => {
//             let mut children = node.children();
//             let (left, dot, right) = (
//                 children.next().expect("left"),
//                 children.next().expect("dot"),
//                 children.next().expect("right"),
//             );
//             assert_eq!(dot.text(), ".");
//             let dic0 = access_module(left, dic)?;
//             let right_name = right.text().to_string();
//             let dic1 = dic0.modules.get(&right_name).ok_or(ToMathlogError {
//                 kind: ToMathlogErrorKind::UnsupportedModule(right_name),
//             })?;
//             Ok(dic1)
//         }
//         _ => todo!("for typst::SyntaxKind::{:#?}", node.kind()),
//     }
//     // println!("access_module: {:#?}", node.children()); // DEBUG

//     // // a.b.c.d === {{a.b}.c}.d
//     // let mut children = node.children();
//     // let (left, dot, right) = (
//     //     children.next().expect("left"),
//     //     children.next().expect("dot"),
//     //     children.next().expect("right"),
//     // );
//     // assert_eq!(dot.text(), ".");
//     // match left.kind() {
//     //     typst::SyntaxKind::MathIdent => {
//     //         let left_name = left.text().to_string();
//     //         let dic0 = dic.modules.get(&left_name).ok_or(ToMathlogError {
//     //             kind: ToMathlogErrorKind::UnsupportedModule(left_name),
//     //         })?;
//     //         access_module(right, dic0)
//     //     }
//     //     typst::SyntaxKind::FieldAccess => {
//     //         let dic0 = access_module(left, dic)?;
//     //         access_module(right, dic0)
//     //     }
//     //     _ => todo!("for typst::SyntaxKind::{:#?}", left.kind()),
//     // }
// }

// // impl mathlog::MathSegment {
// //     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {}
// // }

// impl mathlog::HTMLTag {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Emph {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Link {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Table {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Image {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Env {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Enum {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::Quote {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// impl mathlog::CodeBlock {
//     fn from_typst(node: &typst::SyntaxNode) -> ToMathlogResult<Self> {
//         todo!()
//     }
// }

// fn lookup_ident(s: &str, dic: &Dictionary) -> ToMathlogResult<mathlog::MathSegment> {
//     let name = dic.idents.get(s).ok_or_else(|| ToMathlogError {
//         kind: ToMathlogErrorKind::UnsupportedMathIdent(s.to_string()),
//     })?;
//     Ok(mathlog::MathSegment::Single(name.to_string()))
// }
