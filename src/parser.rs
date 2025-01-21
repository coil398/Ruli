use crate::expr::Expr;
use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric0, char, digit1, multispace0, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<i64>().map(Expr::Number)
    })(input)
}

fn parse_float(input: &str) -> IResult<&str, Expr> {
    map_res(
        recognize(tuple((opt(char('-')), digit1, char('.'), digit1))),
        |s: &str| s.parse::<f64>().map(Expr::Float),
    )(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Expr> {
    map(
        alt((
            recognize(tuple((alpha1, alphanumeric0))),
            recognize(many1(one_of("+-*/=<>!?"))),
        )),
        |s: &str| Expr::Symbol(s.to_string()),
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            preceded(multispace0, char('(')),
            many0(preceded(multispace0, parse_expr)),
            preceded(multispace0, char(')')),
        ),
        Expr::List,
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_list, parse_float, parse_number, parse_symbol))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("123"), Ok(("", Expr::Number(123))));
        assert_eq!(parse_number("-456"), Ok(("", Expr::Number(-456))));
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(parse_float("123.456"), Ok(("", Expr::Float(123.456))));
        assert_eq!(parse_float("-456.789"), Ok(("", Expr::Float(-456.789))));
    }

    #[test]
    fn test_parse_symbol() {
        assert_eq!(
            parse_symbol("foo"),
            Ok(("", Expr::Symbol("foo".to_string())))
        );
        assert_eq!(
            parse_symbol("bar123"),
            Ok(("", Expr::Symbol("bar123".to_string())))
        );
        assert_eq!(parse_symbol("+"), Ok(("", Expr::Symbol("+".to_string()))));
        assert_eq!(parse_symbol("++"), Ok(("", Expr::Symbol("++".to_string()))));
        assert_eq!(parse_symbol("-"), Ok(("", Expr::Symbol("-".to_string()))));
    }

    #[test]
    fn test_parse_list() {
        assert_eq!(
            parse_list("(+ 1 2)"),
            Ok((
                "",
                Expr::List(vec![
                    Expr::Symbol("+".to_string()),
                    Expr::Number(1),
                    Expr::Number(2)
                ])
            ))
        );
        assert_eq!(
            parse_list("(* 3 4)"),
            Ok((
                "",
                Expr::List(vec![
                    Expr::Symbol("*".to_string()),
                    Expr::Number(3),
                    Expr::Number(4),
                ])
            ))
        );
        assert_eq!(
            parse_list("(- 10 5 2)"),
            Ok((
                "",
                Expr::List(vec![
                    Expr::Symbol("-".to_string()),
                    Expr::Number(10),
                    Expr::Number(5),
                    Expr::Number(2),
                ])
            ))
        );
    }
}
