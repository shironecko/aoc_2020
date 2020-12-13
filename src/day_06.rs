use crate::common::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day {
    bags_spec: HashMap<String, Vec<BagChild>>,
}

impl AocDay for Day {
    fn parse_input(&mut self, input: &str) {
        self.bags_spec = input
            .lines()
            .filter_map(|line| bag_parser().parse(line).ok())
            .map(|(_, bag)| (bag.color, bag.children))
            .collect();
    }

    fn puzzle_00(&self) -> Option<AocPuzzleAnswer> {
        None
    }

    fn puzzle_01(&self) -> Option<AocPuzzleAnswer> {
        None
    }
}

type BagColor = String;
type BagChild = (u32, BagColor);

#[derive(Debug, Eq, PartialEq)]
struct BagContents {
    color: BagColor,
    children: Vec<BagChild>,
}

fn color_parser<'a>() -> impl Parser<'a, String> {
    join(
        one_or_more_until(left(word(), space_1()), match_literal("bag")),
        " ",
    )
}

fn child_parser<'a>() -> impl Parser<'a, BagChild> {
    pair(left(number(), space_1()), color_parser())
}

fn children_parser<'a>() -> impl Parser<'a, Vec<BagChild>> {
    one_or_more(left(
        child_parser(),
        pair(
            either(match_literal("bags"), match_literal("bag")),
            either(match_literal(", "), match_literal(".")),
        ),
    ))
}

fn bag_parser<'a>() -> impl Parser<'a, BagContents> {
    map(
        pair(
            left(color_parser(), match_literal("bags contain ")),
            children_parser(),
        ),
        |(color, children)| BagContents { color, children },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_colors_parsing() {
        let input = "pale turquoise bags contain 3 muted cyan bags, 5 striped teal bags.";
        assert_eq!(
            Ok((
                "bags contain 3 muted cyan bags, 5 striped teal bags.",
                "pale turquoise".into()
            )),
            color_parser().parse(input)
        );
        assert_eq!(
            Ok(("bags.", (5, "striped teal".into()))),
            child_parser().parse("5 striped teal bags.")
        );
        assert_eq!(
            Ok(("", vec![(5, "striped teal".into())])),
            children_parser().parse("5 striped teal bags.")
        );
        assert_eq!(
            Ok((
                "",
                BagContents {
                    color: "pale turquoise".into(),
                    children: vec![(3, "muted cyan".into()), (5, "striped teal".into())]
                }
            )),
            bag_parser().parse(input)
        );
    }
}
