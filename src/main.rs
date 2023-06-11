pub mod mathlog_v2;
pub mod typst_;
pub mod utils;

use mathlog_v2 as mathlog;
use typst::syntax::ast::AstNode;

//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use typst::syntax;

    let dic = mathlog::Dictionary::read("./dictionary.json")?;

    let input = r#"
= Greeting; say hello $x + y$ ! $(e)^(i pi) + 1 = 0$ ! $a_i^n$
==
===
Hello, *World* !
This is an example of a _mathematical equation_: $e^(i pi.alt) + 1 = 0$
$
    e^(i π) + 1 = 0
$

WTF!
    "#
    .trim();
    //     let input = r#"
    // = Greeting; say hello $(e)^(i pi) + 1 = 0$ ! $a_i^n$
    // ==
    // ===
    // Hello, *World* !
    //     "#
    // .trim();
    let typst_stx = syntax::ast::Markup::from_untyped(&syntax::parse(input)).unwrap();
    println!("Typst:\n\n{:#?}\n", typst_stx);

    let mathlog_stx = mathlog::ast::Syntax::from_typst(&typst_stx, &dic)?;
    println!("Mathlog:\n\n{:#?}\n", mathlog_stx);
    let output = mathlog_stx.compose();
    println!("output:\n\n{}\n", output);
    Ok(())
}
