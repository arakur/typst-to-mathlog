pub mod mathlog;
pub mod utils;

use typst::syntax::ast::AstNode;

//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use typst::syntax;

    let dic = mathlog::Dictionary::read("./dictionary/dictionary.json")?;

    let input_path = std::env::args().nth(1).expect("No input path");
    // let input_path = "./example/example.typ";
    let input = std::fs::read_to_string(input_path)?;

    let output_path = std::env::args().nth(2).expect("No output path");
    // let output_path = "./example/example.md";

    let typst_stx = syntax::ast::Markup::from_untyped(&syntax::parse(&input))
        .ok_or("parse error in Typst code")?;

    let mathlog_stx = mathlog::ast::Syntax::from_typst(&typst_stx, &dic)?;

    let output = mathlog_stx.compose();

    std::fs::write(output_path, output)?;

    Ok(())
}
