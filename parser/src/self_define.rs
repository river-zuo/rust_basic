
fn hello(input: &str) -> Result<(&str, &str), &str> {
    if input.starts_with("hello") {
        Ok((&input["hello".len()..], "hello"))
    } else {
        Err("parse fail")
    }
}

// type Parser_1 = impl FnMut(&str) -> Result<(&str, &str), &str>;
// type Parser_1 = dyn FnMut(&str) -> Result<(&str, & str), &str>;

pub(crate) fn label<'a>(lab: &'a str) -> impl Fn(&str) -> Result<(&str, &'a str), &str> {
    move |input| {
        if input.starts_with(lab) {
            Ok((&input[lab.len()..], lab))
        } else {
            Err("parse fail")
        }
    }
}

