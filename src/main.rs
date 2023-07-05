pub mod mathlog;
pub mod utils;

use typst::syntax::ast::AstNode;

//

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use typst::syntax;

    let current_exe = std::env::current_exe()?.to_str().unwrap().to_string();

    let dictionary_path = "../../dictionary/dictionary.json".to_string();
    let dictionary_path = current_exe + "\\" + &*dictionary_path;

    let dic = mathlog::Dictionary::read(&dictionary_path)?;

    let current_dir = std::env::current_dir()?.to_str().unwrap().to_string();

    let input_path = std::env::args().nth(1).expect("No input path");
    // let input_path = "./example/example.typ".to_string();
    let input_path = if input_path.starts_with("C:\\") || input_path.starts_with("D:\\") {
        input_path
    } else {
        current_dir.clone() + "\\" + &*input_path
    };
    let input = std::fs::read_to_string(input_path)?;

    let output_path = std::env::args().nth(2).expect("No output path");
    // let output_path = "./example/example.md".to_string();
    let output_path = if output_path.starts_with("C:\\") || output_path.starts_with("D:\\") {
        output_path
    } else {
        current_dir + "\\" + &*output_path
    };

    let typst_stx = syntax::ast::Markup::from_untyped(&syntax::parse(&input))
        .ok_or("parse error in Typst code")?;

    let mathlog_stx = mathlog::ast::Syntax::from_typst(&typst_stx, &dic)?;

    let output = mathlog_stx.compose();

    std::fs::write(output_path, output)?;

    Ok(())
}
