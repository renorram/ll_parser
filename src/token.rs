use crate::grammar::Grammar;

pub const EPSILON: &str = "£";
pub const DOLLAR_SIGN: &str = "$";

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Token {
    Variable(char),
    Terminal(String),
    Epsilon,
    DollarSign
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Variable(ch) => {
                let mut s = String::new();
                s.push(ch.to_owned());
                s
            }
            Token::Terminal(s) => s.to_owned(),
            Token::Epsilon => EPSILON.to_string(),
            Token::DollarSign => DOLLAR_SIGN.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DerivationTokenSlice {
    pub tokens: Vec<Token>,
}

impl DerivationTokenSlice {
    fn new(tokens: Vec<Token>) -> DerivationTokenSlice {
        DerivationTokenSlice { tokens }
    }
}

#[derive(Debug)]
pub struct TokenProcessor<'a> {
    grammar: &'a Grammar
}

impl TokenProcessor<'_> {
    pub fn new(grammar: &Grammar) -> TokenProcessor {
        TokenProcessor { grammar }
    }

    pub fn process_derivation(&self, derivation: &String) -> Vec<DerivationTokenSlice> {
        derivation
            .split("|")
            .map(|slice| DerivationTokenSlice::new(self.get_token_vec(slice.trim())))
            .collect()
    }

    fn get_token_vec(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut buffer = String::new();

        for ch in input.chars() {
            buffer.push(ch);
            if buffer.as_str().eq(EPSILON) {
                tokens.push(Token::Epsilon);
                buffer.clear()
            } else if self.grammar.is_variable(&ch) {
                tokens.push(Token::Variable(ch));
                buffer.clear()
            } else if self.grammar.is_terminal(&buffer) {
                tokens.push(Token::Terminal(buffer.clone()));
                buffer.clear()
            }
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::Grammar;
    use crate::token::{DerivationTokenSlice, Token, TokenProcessor};

    #[test]
    fn test_get_token_vec() {
        let grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
            initial_symbol: 'S'
        };

        let derivation_slice = String::from("bB");
        let expected = vec![Token::Terminal("b".to_string()), Token::Variable('B')];

        let processor = TokenProcessor::new(&grammar);

        assert_eq!(processor.get_token_vec(&derivation_slice), expected)
    }

    #[test]
    fn test_get_derivations() {
        let grammar = Grammar {
            variables: vec!['E', 'Z', 'T', 'Y', 'X'],
            terminals: vec![
                "+".to_string(),
                "*".to_string(),
                "(".to_string(),
                "id".to_string(),
                ")".to_string(),
            ],
            productions: vec![],
            initial_symbol: 'S'
        };

        let processor = TokenProcessor::new(&grammar);

        let deriv1 = String::from("*XY | £");
        let deriv2 = String::from("(E) | id");

        let derivation1_slices: Vec<DerivationTokenSlice> = vec![
            DerivationTokenSlice::new(vec![
                Token::Terminal("*".to_string()),
                Token::Variable('X'),
                Token::Variable('Y'),
            ]),
            DerivationTokenSlice::new(vec![Token::Epsilon]),
        ];

        let derivation2_slices: Vec<DerivationTokenSlice> = vec![
            DerivationTokenSlice::new(vec![
                Token::Terminal("(".to_string()),
                Token::Variable('E'),
                Token::Terminal(")".to_string()),
            ]),
            DerivationTokenSlice::new(vec![Token::Terminal("id".to_string())]),
        ];

        assert_eq!(processor.process_derivation(&deriv1), derivation1_slices);
        assert_eq!(processor.process_derivation(&deriv2), derivation2_slices);
    }
}
