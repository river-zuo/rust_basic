use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::{self, Error, ErrorKind, ParseError};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, separated_pair, terminated, tuple};
use nom::{Err, IResult, Parser as Pa};

trait Parser {
    fn parse(&mut self, input: &str) -> Result<(&str, &str), &str>;
}


// fn hello_parser<I, O, E>(input: I) -> Result<(I, O), E> {
//     if input.starts_with("hello") {
//         Ok((&input["hello".len()..], "hello"))
//     } else {
//         Err("parse fail")
//     }
// }

fn _parse() {
    let s = "Hello, world!";
    println!("{}", &s["he".len()..]);

    assert_eq!(crate::self_define::label("Hello")("Hello, world!"), Ok((", world!", "Hello")));
    println!("Hello, world!");
    
    fn parser_0(input: &str) -> IResult<&str, &str> {
        tag("Hello")(input)
    }
    fn parser_comma(input: &str) -> IResult<&str, &str> {
        tag(", ")(input)
    }

    fn parser_world(input: &str) -> IResult<&str, &str> {
        tag("world")(input)
    }
    
    fn ac_parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
        match parser_0(input) {
            Ok((input, output_hello)) => {
                match parser_comma(input) {
                    Ok((input, output_common)) => {
                        match parser_world(input) {
                            Ok((input, output_world)) => Ok((input, (output_hello, output_common, output_world))),
                            Result::Err(e) => Err(e),
                        }
                    },
                    Result::Err(e) => Err(e),
                }
            },
            Result::Err(e) => Err(e),
        }
    }

    // assert_eq!(parser_0("Hello, world!"), Ok((", world!", "Hello")));

    // let p0 = parser_0("Hello, world!");
    // println!("{:?}", p0);

    fn pp0(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        pair(tag("Hello"), pair(tag(", "), tag("world")))(input)
    }

    fn pp1(input: &str) -> IResult<&str, (&str, &str, &str)> {
        tuple((tag("Hello"), tag(", "), tag("world")))(input)
    }

    // let p0 = ac_parser("Hello, world!");
    // println!("{:?}", p0);

    let p0 = pp0("Hello, world!");
    println!("pp0_ {:?}", p0);
    
    let p1 = pp1("Hello, world!");
    println!("pp1_ {:?}", p1);
    

}


fn parse_alt(input: &str) -> IResult<&str, &str> {
    alt((tag("hello"), tag("world")))(input)
}

fn parse_delimited(input: &str) -> IResult<&str, &str> {
    delimited(tag("("), tag("hello"), tag(")"))(input)
}

fn parse_separated(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag("key"), tag("="), tag("value"))(input)
}

fn parse_preced(input: &str) -> IResult<&str, &str> {
    preceded(tag("database="), tag("student"))(input)
}

fn parse_terminal(input: &str) -> IResult<&str, &str> {
    terminated(tag("hello"), tag("!"))(input)
}

fn parse_alpha(input: &[u8]) -> IResult<&[u8], char> {
    // let i = &input[0..1];
    // input.as_bytes()
    let i_char = input.get(0).copied().unwrap_or_default();
    if i_char.is_ascii_alphabetic() {
        return  Ok((&input[1..], i_char as char));
    } else {
        // return Err(ParseError::from_error_kind(input, ErrorKind::Alpha));
        return Err(Err::Error(nom::error::Error::new(input, ErrorKind::Alpha)));
    }
}

pub(crate) fn parse_many(input: &[u8]) -> IResult<&[u8], Vec<char>> {
    many0(parse_alpha)(input)
}

fn _parse0() {
    // let input = "hello, world";
    // let input = "world,sss";
    // let p0 = parse_alt(input);
    // println!("{:?}", p0);
    
    // let input = "(hello)";
    // let p1 = parse_delimited(input);
    // println!("p1_ {:?}", p1);

    // let input = "key=value";
    // let ps0 = parse_separated(input);
    // println!("{:?}", ps0);

    // let input = "database=student";
    // let ps0 = parse_prece(input);
    // println!("{:?}", ps0);

}
