pub trait Intercalate<S, T> {
    fn intercalate(self, sep: S) -> T;
}

impl<I> Intercalate<&'_ str, String> for I
where
    I: IntoIterator<Item = String>,
{
    fn intercalate(self, sep: &str) -> String {
        self.into_iter().collect::<Vec<_>>().join(sep)
    }
}

#[test]
fn _test() {
    let v = vec!["a", "b", "c"];
    let s = v.iter().map(|s| s.to_string()).intercalate(", ");
    assert_eq!(s, "a, b, c");
}

pub fn add_indent(s: &str, init_indent_num: usize, rest_indent_num: usize) -> String {
    s.lines()
        .enumerate()
        .map(|(i, line)| {
            let indent_num = if i == 0 {
                init_indent_num
            } else {
                rest_indent_num
            };
            format!("{}{}", " ".repeat(indent_num), line)
        })
        .intercalate("\n")
}
