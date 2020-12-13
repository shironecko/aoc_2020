///
/// made with guidance of excellent https://bodil.lol/parser-combinators/
///
use std::iter::once;

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

#[allow(dead_code)]
pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(slice) if slice == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

#[allow(dead_code)]
pub fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

#[allow(dead_code)]
pub fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

#[allow(dead_code)]
pub fn one_or_more_until<'a, P1, P2, Output, Discard>(
    parser_1: P1,
    parser_2: P2,
) -> impl Parser<'a, Vec<Output>>
where
    P1: Parser<'a, Output>,
    P2: Parser<'a, Discard>,
{
    move |input: &'a str| {
        if let Ok(_) = parser_2.parse(input) {
            return Err(input);
        }

        let (mut input_cursor, first_result) = parser_1.parse(input)?;
        let mut results = vec![first_result];
        while let Err(_) = parser_2.parse(input_cursor) {
            let (next_input, result) = parser_1.parse(input_cursor)?;
            input_cursor = next_input;
            results.push(result);
        }

        Ok((input_cursor, results))
    }
}

#[allow(dead_code)]
pub fn zero_or_more_until<'a, P1, P2, Output, Discard>(
    parser_1: P1,
    parser_2: P2,
) -> impl Parser<'a, Vec<Output>>
where
    P1: Parser<'a, Output>,
    P2: Parser<'a, Discard>,
{
    move |input: &'a str| {
        let mut input_cursor = input;
        let mut results = Vec::new();
        while let Err(_) = parser_2.parse(input_cursor) {
            let (next_input, result) = parser_1.parse(input_cursor)?;
            input_cursor = next_input;
            results.push(result);
        }

        Ok((input_cursor, results))
    }
}

#[allow(dead_code)]
pub fn pair<'a, P1, P2, R1, R2>(parser_1: P1, parser_2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        let (tail, result_1) = parser_1.parse(input)?;
        let (tail, result_2) = parser_2.parse(tail)?;
        Ok((tail, (result_1, result_2)))
    }
}

#[allow(dead_code)]
pub fn either<'a, P1, P2, Output>(parser_1: P1, parser_2: P2) -> impl Parser<'a, Output>
where
    P1: Parser<'a, Output>,
    P2: Parser<'a, Output>,
{
    move |input| match parser_1.parse(input) {
        result @ Ok(_) => result,
        Err(_) => parser_2.parse(input),
    }
}

#[allow(dead_code)]
pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(tail, output)| (tail, map_fn(output)))
    }
}

#[allow(dead_code)]
pub fn map_opt<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> Option<B>,
{
    move |input| {
        let (tail, result) = parser.parse(input)?;
        if let Some(mapped) = map_fn(result) {
            Ok((tail, mapped))
        } else {
            Err(input)
        }
    }
}

#[allow(dead_code)]
pub fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

#[allow(dead_code)]
pub fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

#[allow(dead_code)]
pub fn any_char<'a>() -> impl Parser<'a, char> {
    move |input: &'a str| match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

#[allow(dead_code)]
pub fn pred<'a, P, R, F>(parser: P, predicate: F) -> impl Parser<'a, R>
where
    P: Parser<'a, R>,
    F: Fn(&R) -> bool,
{
    move |input| {
        let (input_tail, result) = parser.parse(input)?;
        if predicate(&result) {
            Ok((input_tail, result))
        } else {
            Err(input)
        }
    }
}

#[allow(dead_code)]
pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char(), |c| c.is_whitespace())
}

#[allow(dead_code)]
pub fn space_1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

#[allow(dead_code)]
pub fn space_0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

#[allow(dead_code)]
pub fn quoted_string<'a>() -> impl Parser<'a, String> {
    map(
        right(
            match_literal("\""),
            left(
                zero_or_more(pred(any_char(), |c| *c != '"')),
                match_literal("\""),
            ),
        ),
        |chars| chars.into_iter().collect(),
    )
}

#[allow(dead_code)]
pub fn word<'a>() -> impl Parser<'a, String> {
    map(
        one_or_more(pred(any_char(), |c| c.is_alphabetic())),
        |chars| chars.into_iter().collect(),
    )
}

#[allow(dead_code)]
pub fn join<'a, P: Parser<'a, Vec<String>>>(
    parser: P,
    separator: &'static str,
) -> impl Parser<'a, String> {
    map(parser, move |vector| vector.join(separator))
}

#[allow(dead_code)]
pub fn identifier<'a>() -> impl Parser<'a, String> {
    move |input: &'a str| match input.chars().next() {
        Some(head) if head.is_alphabetic() => {
            let tail = input
                .chars()
                .skip(1)
                .take_while(|&c| c.is_alphanumeric() || ['-', '_'].contains(&c));
            let ident: String = once(head).chain(tail).collect();
            Ok((&input[ident.len()..], ident))
        }
        _ => Err(input),
    }
}

#[allow(dead_code)]
pub fn number<'a>() -> impl Parser<'a, u32> {
    map_opt(
        one_or_more(pred(any_char(), |c| c.is_numeric())),
        |digits| digits.into_iter().collect::<String>().parse().ok(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello, Joe!");
        assert_eq!(Ok(("", ())), parse_joe.parse("Hello, Joe!"));
        assert_eq!(
            Ok((" Hello, Robert!", ())),
            parse_joe.parse("Hello, Joe! Hello, Robert!")
        );
        assert_eq!(Err("Hello, Mike!"), parse_joe.parse("Hello, Mike!"));
    }

    #[test]
    fn identifier_parser() {
        assert_eq!(
            Ok(("", "one-identifier_underscore".into())),
            identifier().parse("one-identifier_underscore")
        );
        assert_eq!(
            Ok((" identifiers", "two".into())),
            identifier().parse("two identifiers")
        );
        assert_eq!(
            Err("#no-identifiers"),
            identifier().parse("#no-identifiers")
        );
    }

    #[test]
    fn pair_combinator() {
        let tag_opener = pair(match_literal("<"), identifier());
        assert_eq!(
            Ok(("/>", ((), "my-first-element".into()))),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn either_combinator() {
        let parser = either(match_literal("one"), match_literal("two"));
        assert_eq!(Ok((" two", ())), parser.parse("one two"));
        assert_eq!(Ok((" one", ())), parser.parse("two one"));
        assert_eq!(Err("none"), parser.parse("none"));
    }

    #[test]
    fn left_combinator() {
        let tag_opener = left(match_literal("<"), identifier());
        assert_eq!(Ok(("/>", ())), tag_opener.parse("<my-first-element/>"));
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn right_combinator() {
        let tag_opener = right(match_literal("<"), identifier());
        assert_eq!(
            Ok(("/>", "my-first-element".into())),
            tag_opener.parse("<my-first-element/>")
        );
        assert_eq!(Err("oops"), tag_opener.parse("oops"));
        assert_eq!(Err("!oops"), tag_opener.parse("<!oops"));
    }

    #[test]
    fn one_or_more_combinator() {
        let parser = one_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
        assert_eq!(Err("ahah"), parser.parse("ahah"));
        assert_eq!(Err(""), parser.parse(""));
    }

    #[test]
    fn zero_or_more_combinator() {
        let parser = zero_or_more(match_literal("ha"));
        assert_eq!(Ok(("", vec![(), (), ()])), parser.parse("hahaha"));
        assert_eq!(Ok(("ahah", vec![])), parser.parse("ahah"));
        assert_eq!(Ok(("", vec![])), parser.parse(""));
    }

    #[test]
    fn one_or_more_until_combinator() {
        let parser = one_or_more_until(right(space_0(), word()), match_literal(";"));
        assert_eq!(
            Ok((";", vec!["one".into(), "two".into(), "three".into()])),
            parser.parse("one two three;")
        );
        assert_eq!(Err(";nothing"), parser.parse(";nothing"));
        assert_eq!(Err("!error two;"), parser.parse("one !error two;"));
    }

    #[test]
    fn zero_or_more_until_combinator() {
        let parser = zero_or_more_until(right(space_0(), word()), match_literal(";"));
        assert_eq!(
            Ok((";", vec!["one".into(), "two".into(), "three".into()])),
            parser.parse("one two three;")
        );
        assert_eq!(Ok((";nothing", Vec::new())), parser.parse(";nothing"));
        assert_eq!(Err("!error two;"), parser.parse("one !error two;"));
    }

    #[test]
    fn predicate_combinator() {
        let parser = pred(any_char(), |c| *c == 'o');
        assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
        assert_eq!(Err("lol"), parser.parse("lol"));
    }

    #[test]
    fn quoted_string_parser() {
        assert_eq!(
            Ok(("", "Hello Joe!".into())),
            quoted_string().parse("\"Hello Joe!\"")
        );
    }

    #[test]
    fn word_parser() {
        assert_eq!(
            Ok((" quick brown fox", "a".into())),
            word().parse("a quick brown fox")
        );
        assert_eq!(Ok((" word", "first".into())), word().parse("first word"));
        assert_eq!(Ok(("", "onlyWord".into())), word().parse("onlyWord"));
        assert_eq!(Err("~no-words"), word().parse("~no-words"));
    }

    #[test]
    fn join_combinator() {
        assert_eq!(
            Ok(("", "one two three".into())),
            join(one_or_more(left(word(), space_0())), " ").parse("one two three")
        );
    }

    #[test]
    fn number_parser() {
        assert_eq!(Ok(("", 42)), number().parse("42"));
        assert_eq!(
            Ok((" and then some", 16746)),
            number().parse("16746 and then some")
        );
        assert_eq!(Err("NaN"), number().parse("NaN"));
    }
}
