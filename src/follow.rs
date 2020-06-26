use crate::grammar::Grammar;
use crate::production::Production;
use crate::token::{Token, TokenProcessor};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Item {
    variable: char,
    token: Token,
}

impl Item {
    fn new(variable: char, token: Token) -> Item {
        Item { variable, token }
    }
}

impl Production {
    fn process_follow_variable(
        variable: char,
        grammar: &Grammar,
        next_token: Option<&Token>,
    ) -> Vec<Item> {
        let mut tokens: Vec<Item> = vec![];

        if let Some(token) = next_token {
            match token {
                Token::Variable(ch) => {
                    for token in Self::process_follow_rule2(ch.clone(), grammar) {
                        tokens.push(Item::new(variable, token))
                    }
                }
                Token::Terminal(_) => tokens.push(Item::new(variable, token.clone())),
                _ => (),
            }
        }

        return tokens;
    }

    fn process_follow_rule2(variable: char, grammar: &Grammar) -> HashSet<Token> {
        let prod = grammar.get_production_by_var(variable).unwrap();

        prod.firsts
            .iter()
            .filter_map(|t| {
                if !t.eq(&Token::Epsilon) {
                    return Some(t.clone());
                }

                None
            })
            .collect()
    }

    fn process_follow_rule3(next_token: Option<&Token>, grammar: &Grammar) -> bool {
        if let Some(token) = next_token {
            match token {
                Token::Variable(ch) => {
                    let prod = grammar.get_production_by_var(ch.clone()).unwrap();

                    return prod.firsts.contains(&Token::Epsilon);
                }
                _ => (),
            }
        }

        false
    }

    fn fetch_follows(
        production: &Production,
        grammar: &Grammar,
        token_processor: &TokenProcessor,
    ) -> Vec<Item> {
        let mut tokens: Vec<Item> = vec![];

        // 1 regra
        if grammar.production_is_initial(production) {
            tokens.push(Item::new(production.variable, Token::DollarSign));
        }

        let derivations = token_processor.process_derivation(&production.derivation);

        for slice in derivations {
            let slice_size = slice.tokens.len();

            for (index, token) in slice.tokens.iter().enumerate() {
                match token {
                    Token::Variable(ch) => {
                        let var_tokens = Self::process_follow_variable(
                            ch.clone(),
                            grammar,
                            slice.tokens.get(index + 1),
                        );
                        tokens = [tokens, var_tokens].concat();

                        // check rule 3
                        if index + 1 == slice_size {
                            tokens.push(Item::new(
                                ch.clone(),
                                Token::Placeholder(production.variable),
                            ))
                        }

                        // check rule 3 part 2
                        if index + 2 == slice_size {
                            if Self::process_follow_rule3(slice.tokens.get(index + 1), grammar) {
                                tokens.push(Item::new(
                                    ch.clone(),
                                    Token::Placeholder(production.variable),
                                ))
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        tokens
    }
}

impl Grammar {
    pub fn compute_follows(&mut self) {
        self.compute_firsts();

        let immut_self = self.clone();
        let processor = TokenProcessor::new(&immut_self);
        let tokens: Vec<Item> = self
            .productions_iter_mut()
            .map(|p| Production::fetch_follows(p, &immut_self, &processor))
            .flatten()
            .collect();

        tokens
            .iter()
            .filter(|item| match item.token {
                Token::Placeholder(_) => false,
                _ => true,
            })
            .for_each(|item| {
                self.get_mut_production_by_var(item.variable)
                    .unwrap()
                    .follows
                    .insert(item.token.clone());
            });

        // clear placeholders
        tokens
            .iter()
            .filter(|item| match item.token {
                Token::Placeholder(_) => true,
                _ => false,
            })
            .for_each(|item| {
                let mut cl = self.clone();
                let item_prod = self.get_mut_production_by_var(item.variable).unwrap();

                match item.token {
                    Token::Placeholder(ch) => {
                        let placeholder_follows = cl.get_mut_production_by_var(ch).unwrap();
                        for f in placeholder_follows.follows.iter() {
                            item_prod.follows.insert(f.clone());
                        }
                    }
                    _ => (),
                }
            });
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::Grammar;
    use crate::production::Production;
    use crate::token::{Token, DOLLAR_SIGN, EPSILON};
    use std::collections::HashSet;

    fn hash_from_vec(vec: Vec<&str>) -> HashSet<Token> {
        vec.iter()
            .map(|&v| match v {
                EPSILON => Token::Epsilon,
                DOLLAR_SIGN => Token::DollarSign,
                _ => Token::Terminal(v.to_string()),
            })
            .collect()
    }

    #[test]
    fn test_follow() {
        let mut grammar = Grammar {
            variables: vec!['E', 'Z', 'T', 'Y', 'F'],
            terminals: vec![
                "+".to_string(),
                "*".to_string(),
                "(".to_string(),
                "id".to_string(),
                ")".to_string(),
            ],
            productions: vec![],
            initial_symbol: 'E',
        };

        grammar.add_production(Production::new('E', "TZ".to_string()).unwrap());
        grammar.add_production(Production::new('Z', "+TZ | £".to_string()).unwrap());
        grammar.add_production(Production::new('T', "FY".to_string()).unwrap());
        grammar.add_production(Production::new('Y', "*FY | £".to_string()).unwrap());
        grammar.add_production(Production::new('F', "(E) | id".to_string()).unwrap());

        grammar.compute_follows();

        let e = grammar.get_production_by_var('E').unwrap();
        let z = grammar.get_production_by_var('Z').unwrap();
        let t = grammar.get_production_by_var('T').unwrap();
        let y = grammar.get_production_by_var('Y').unwrap();
        let f = grammar.get_production_by_var('F').unwrap();

        let set_e = hash_from_vec(vec![DOLLAR_SIGN, ")"]);
        let set_t = hash_from_vec(vec!["+", ")", DOLLAR_SIGN]);
        let set_f = hash_from_vec(vec!["*", "+", ")", DOLLAR_SIGN]);

        assert_eq!(e.follows, set_e, "Testing variable E");
        assert_eq!(z.follows, set_e, "Testing variable Z");
        assert_eq!(t.follows, set_t, "Testing variable T");
        assert_eq!(y.follows, set_t, "Testing variable Y");
        assert_eq!(f.follows, set_f, "Testing variable F");
    }

    #[test]
    fn test_follow2() {
        let mut grammar = Grammar {
            variables: vec!['S', 'B', 'C'],
            terminals: vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
            ],
            productions: vec![],
            initial_symbol: 'S',
        };

        grammar.add_production(Production::new('S', "Bb | Cd".to_string()).unwrap());
        grammar.add_production(Production::new('B', "aB | £".to_string()).unwrap());
        grammar.add_production(Production::new('C', "cC | £".to_string()).unwrap());

        grammar.compute_follows();

        let s = grammar.get_production_by_var('S').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        let c = grammar.get_production_by_var('C').unwrap();

        let set_s = hash_from_vec(vec![DOLLAR_SIGN]);
        let set_b = hash_from_vec(vec!["b"]);
        let set_c = hash_from_vec(vec!["d"]);

        assert_eq!(s.follows, set_s, "Testing variable S");
        assert_eq!(b.follows, set_b, "Testing variable B");
        assert_eq!(c.follows, set_c, "Testing variable C");
    }
}
