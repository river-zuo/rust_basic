use std::error;

use nom::{bytes::complete::tag, combinator::{consumed, map, peek, recognize}, error::Error, multi::{many0, many1}, sequence::delimited, Err, IResult};

mod self_define;
mod parse_combine;


fn many0_hello(input: &str) -> IResult<&str, Vec<&str>> {
    many1(tag("hello"))(input)
}

#[test]
fn test_many0_hello() {
    let input = "hellohellohel";
    let input = "aaahellohellohel";
    let hh = many0_hello(input);
    println!("{:?}", hh);

    // Error::new("aaahellohellohel", nom::error::ErrorKind::Tag
    // Err(Err::Error(nom::error::Error::new(input, ErrorKind::Alpha)));
    assert_eq!(hh, Err(Err::Error(nom::error::Error::new("aaahellohellohel", nom::error::ErrorKind::Tag))))

    // let vv = hh.unwrap().1;
    // for iv in 0..vv.len() {
    //     println!("{}_ {}", iv, vv.get(iv).unwrap());
    // }

}

fn peek_hello(input: &str) -> IResult<&str, &str> {
    peek(tag("hello"))(input)
}

fn precognize_delimited_parse(input: &str) -> IResult<&str, &str> {
    recognize(delimited(tag("("), tag("hello"), tag(")")))(input)
}

fn comsume_delimited_parse(input: &str) -> IResult<&str, (&str, &str)> {
    consumed(delimited(tag("("), tag("hello"), tag(")")))(input)
}

fn map_hello_len(input: &str) -> IResult<&str, usize> {
    map(tag("hello"), |s: &str| s.len())(input)
}

#[test]
fn test_1() {
    let input = "helloaaa";
    let re = map_hello_len(input);
    println!("{:?}", re);
    
}


fn main() {

    // let input = "hello!";
    // let ps0 = parse_terminal(input);
    // println!("{:?}", ps0);

    // let input = "hello!";
    // let i = &input[0..1];

    // let input = "hello, world";
    // let output = parse_combine::parse_many(input.as_bytes());
    // println!("{:?}", output);
    
    // let input = "hello, world";
    // let output = peek_hello(input);
    // println!("{:?}", output);
    // assert_eq!(peek_hello(input), Ok((input, "hello")));

    let input = "(hello)";
    let output = precognize_delimited_parse(input);
    println!("{:?}", output);

    let input = "(hello)";
    let output = comsume_delimited_parse(input);
    println!("{:?}", output);


}

