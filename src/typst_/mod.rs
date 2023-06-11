#[test]
fn typst_syntax00() {
    use typst::syntax;
    let input = r#"
= Greeting; say hello $e^(i pi) + 1 = 0$ !
Hello, *World* !
This is an example of a _mathematical equation_: $e^(i pi.alt) + 1 = 0$
$
    e^(i pi) + 1 = 0
$

WTF!
    "#
    .trim();
    let syn = syntax::parse(input);
    println!("{:#?}", syn);
}
