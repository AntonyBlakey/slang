use super::{
    super::{cst, kinds::*},
    stream::Stream,
};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ParserResult {
    Match(Match),
    PrattOperatorMatch(PrattOperatorMatch),
    IncompleteMatch(IncompleteMatch),
    NoMatch(NoMatch),
}

impl ParserResult {
    pub fn r#match(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        ParserResult::Match(Match::new(
            nodes,
            tokens_that_would_have_allowed_more_progress,
        ))
    }

    pub fn pratt_operator_match(nodes: Vec<(u8, Vec<cst::Node>, u8)>) -> Self {
        ParserResult::PrattOperatorMatch(PrattOperatorMatch::new(nodes))
    }

    pub fn incomplete_match(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        ParserResult::IncompleteMatch(IncompleteMatch::new(
            nodes,
            tokens_that_would_have_allowed_more_progress,
        ))
    }

    pub fn no_match(tokens_that_would_have_allowed_more_progress: Vec<TokenKind>) -> Self {
        ParserResult::NoMatch(NoMatch::new(tokens_that_would_have_allowed_more_progress))
    }

    pub fn is_match(&self) -> bool {
        match self {
            ParserResult::Match(_) | ParserResult::PrattOperatorMatch(_) => true,
            _ => false,
        }
    }

    pub fn is_no_match(&self) -> bool {
        match self {
            ParserResult::NoMatch(_) => true,
            _ => false,
        }
    }

    pub fn with_kind(self, new_kind: RuleKind) -> ParserResult {
        match self {
            ParserResult::Match(r#match) => ParserResult::r#match(
                vec![cst::Node::rule(new_kind, r#match.nodes)],
                r#match.tokens_that_would_have_allowed_more_progress,
            ),
            ParserResult::IncompleteMatch(incomplete_match) => ParserResult::incomplete_match(
                vec![cst::Node::rule(new_kind, incomplete_match.nodes)],
                incomplete_match.tokens_that_would_have_allowed_more_progress,
            ),
            ParserResult::NoMatch(_) => self,
            _ => unreachable!("PrattOperatorMatch cannot be converted to a rule"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Match {
    pub nodes: Vec<cst::Node>,
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl Match {
    pub fn new(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            nodes,
            tokens_that_would_have_allowed_more_progress,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PrattOperatorMatch {
    pub nodes: Vec<(u8, Vec<cst::Node>, u8)>,
}

impl PrattOperatorMatch {
    pub fn new(nodes: Vec<(u8, Vec<cst::Node>, u8)>) -> Self {
        Self { nodes }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct IncompleteMatch {
    pub nodes: Vec<cst::Node>,
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl IncompleteMatch {
    pub fn new(
        nodes: Vec<cst::Node>,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            nodes,
            tokens_that_would_have_allowed_more_progress,
        }
    }

    pub fn consume_stream(&self, stream: &mut Stream) {
        for node in &self.nodes {
            for _ in 0..node.text_len().char {
                stream.next();
            }
        }
    }

    pub fn is_better_match_than(&self, other: &IncompleteMatch) -> bool {
        let first_size = self
            .nodes
            .iter()
            .fold(0, |acc, node| acc + node.text_len().utf8);
        let second_size = other
            .nodes
            .iter()
            .fold(0, |acc, node| acc + node.text_len().utf8);
        first_size > second_size
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NoMatch {
    pub tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl NoMatch {
    pub fn new(tokens_that_would_have_allowed_more_progress: Vec<TokenKind>) -> Self {
        Self {
            tokens_that_would_have_allowed_more_progress,
        }
    }
}