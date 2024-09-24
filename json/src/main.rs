use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::{map, recognize},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use nom::bytes::complete::take_while;
use nom::combinator::opt;

#[derive(Debug)]
#[allow(dead_code)]
enum JsonValue {
    Null,
    Num(f64),
    Bool(bool),
    Str(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn parse_null(input: &str) -> IResult<&str, JsonValue> {
    map(tag("null"), |_| JsonValue::Null)(input)
}

fn parse_bool(input: &str) -> IResult<&str, JsonValue> {
    alt((
        map(tag("true"), |_| JsonValue::Bool(true)),
        map(tag("false"), |_| JsonValue::Bool(false)),
    ))(input)
}

fn parse_num(input: &str) -> IResult<&str, JsonValue> {
    map(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        JsonValue::Num(s.parse().unwrap())
    })(input)
}

fn parse_str(input: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(char('"'), take_while(|c| c != '"'), char('"')),
        |s: &str| JsonValue::Str(s.to_string()),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, JsonValue> {
    preceded(
        multispace0,
        alt((
            parse_str,
            parse_num,
            parse_bool,
            parse_null,
            parse_array,
            parse_object,
        )),
    )(input)
}

fn parse_array(input: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('['),
            separated_list0(
                preceded(multispace0, char(',')),
                preceded(multispace0, parse_value),
            ),
            preceded(multispace0, char(']')),
        ),
        JsonValue::Array,
    )(input)
}

fn parse_pair(input: &str) -> IResult<&str, (JsonValue, JsonValue)> {
    separated_pair(
        preceded(multispace0, parse_str),
        preceded(multispace0, char(':')),
        preceded(multispace0, parse_value),
    )(input)
}

fn parse_object(input: &str) -> IResult<&str, JsonValue> {
    map(
        delimited(
            char('{'),
            separated_list1(
                preceded(multispace0, char(',')),
                preceded(multispace0, parse_pair),
            ),
            preceded(multispace0, char('}')),
        ),
        |pairs| {
            JsonValue::Object(
                pairs
                    .into_iter()
                    .map(|(k, v)| {
                        if let JsonValue::Str(key) = k {
                            return (key, v);
                        }
                        panic!("key")
                    })
                    .collect(),
            )
        },
    )(input)
}

fn parse_json(input: &str) -> IResult<&str, JsonValue> {
    terminated(parse_value, multispace0)(input)
}

fn main() {
    // println!("{:?}",parse_null( input:"null"));
    // println!("{:?}", parse_bool( input: "true"));
    // println!("{:?}",parse_num( input:"123"));
    // println!("{:?}", parse_str( input: r#""hello""#));
    // println!("{:?}",parse_array( input: r#"[200,“300",600]"#));
    // println!("{:?}",parse_object( input: r#"{"key": 200}"#));

    let json_str = r#"
        {
            "nickname":"张三",
            "age": 30,
            "is teacher": false,
            "scores":[90,85,95],
            "address":{
                "city":"北京",
                "street":"中关村大街",
                "code":[200,2000]
        }
    }
    "#;

    match parse_json(json_str) {
        Ok((_, json)) => println!("{:#?}", json),
        Err(e) => println!("Error:{:?}", e),
    }
}
