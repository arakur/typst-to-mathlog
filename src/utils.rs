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

//

pub fn roman(i: usize) -> String {
    if i == 0 {
        return "0".to_string();
    }
    let mut s = String::new();
    let mut i = i;
    while i > 0 {
        if i >= 1000 {
            s.push('M');
            i -= 1000;
        } else if i >= 900 {
            s.push_str("CM");
            i -= 900;
        } else if i >= 500 {
            s.push('D');
            i -= 500;
        } else if i >= 400 {
            s.push_str("CD");
            i -= 400;
        } else if i >= 100 {
            s.push('C');
            i -= 100;
        } else if i >= 90 {
            s.push_str("XC");
            i -= 90;
        } else if i >= 50 {
            s.push('L');
            i -= 50;
        } else if i >= 40 {
            s.push_str("XL");
            i -= 40;
        } else if i >= 10 {
            s.push('X');
            i -= 10;
        } else if i >= 9 {
            s.push_str("IX");
            i -= 9;
        } else if i >= 5 {
            s.push('V');
            i -= 5;
        } else if i >= 4 {
            s.push_str("IV");
            i -= 4;
        } else if i >= 1 {
            s.push('I');
            i -= 1;
        } else {
            unreachable!()
        }
    }
    s
}
