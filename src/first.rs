use super::grammar::Grammar;
use super::production::Production;
use crate::token::{Token, TokenProcessor};
use std::collections::HashSet;

impl Production {
    fn process_variable(
        variable: char,
        grammar: &Grammar,
        firsts: &mut HashSet<Token>,
        production: &Production,
    ) -> bool {
        if let Some(p) = grammar.get_production_by_var(variable) {
            let variable_firsts = Self::fetch_firsts(p, grammar);
            // only continue processing derivation if the variable firsts contains an epsilon
            let should_continue = variable_firsts.contains(&Token::Epsilon);

            for token in variable_firsts {
                match token {
                    // rule 3.a
                    Token::Epsilon => {
                        production.ends_with_variable(variable) && firsts.insert(token)
                    }
                    _ => firsts.insert(token),
                };
            }

            return should_continue;
        }

        false
    }

    fn fetch_firsts(production: &Production, grammar: &Grammar) -> HashSet<Token> {
        let mut firsts: HashSet<Token> = HashSet::new();
        let processor = TokenProcessor::new(grammar);

        for slice in processor.process_derivation(&production.derivation) {
            for token in slice.tokens {
                let should_continue = match token {
                    Token::Variable(ch) => {
                        Self::process_variable(ch, grammar, &mut firsts, &production)
                    }
                    _ => {
                        firsts.insert(token);
                        false
                    }
                };

                if !should_continue {
                    break;
                }
            }
        }

        firsts
    }
}

impl Grammar {
    pub fn compute_firsts(&mut self) {
        let immut_self = self.clone();

        for p in self.productions_iter_mut() {
            p.set_firsts(Production::fetch_firsts(p, &immut_self))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::grammar::Grammar;
    use crate::production::Production;
    use crate::token::{Token, EPSILON};
    use std::collections::HashSet;

    fn hash_from_vec(vec: Vec<&str>) -> HashSet<Token> {
        vec.iter()
            .map(|&v| {
                if v.eq(EPSILON) {
                    Token::Epsilon
                } else {
                    Token::Terminal(v.to_string())
                }
            })
            .collect()
    }

    #[test]
    fn test_firsts_simple() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
            initial_symbol: 'S',
        };

        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c".to_string()).unwrap());

        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        let set_s: HashSet<_> = hash_from_vec(vec!["a"]);
        let set_b: HashSet<_> = hash_from_vec(vec!["b", "c"]);

        assert_eq!(s.firsts, set_s, "Testing variable S");
        assert_eq!(a.firsts, set_s, "Testing variable A");
        assert_eq!(b.firsts, set_b, "Testing variable B");
    }

    #[test]
    fn test_firsts_epsilon() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
            initial_symbol: 'S',
        };

        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a | £".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c".to_string()).unwrap());

        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        let set_s: HashSet<_> = hash_from_vec(vec!["a", "b", "c"]);
        let set_a: HashSet<_> = hash_from_vec(vec!["a", EPSILON]);
        let set_b: HashSet<_> = hash_from_vec(vec!["b", "c"]);

        assert_eq!(s.firsts, set_s, "Testing variable S");
        assert_eq!(a.firsts, set_a, "Testing variable A");
        assert_eq!(b.firsts, set_b, "Testing variable B");
    }

    #[test]
    fn test_firsts_epsilon_2() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
            initial_symbol: 'S',
        };

        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a | £".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c | £".to_string()).unwrap());

        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        let set_s: HashSet<_> = hash_from_vec(vec!["a", "b", "c", EPSILON]);
        let set_a: HashSet<_> = hash_from_vec(vec!["a", EPSILON]);
        let set_b: HashSet<_> = hash_from_vec(vec!["b", "c", EPSILON]);

        assert_eq!(s.firsts, set_s, "Testing variable S");
        assert_eq!(a.firsts, set_a, "Testing variable A");
        assert_eq!(b.firsts, set_b, "Testing variable B");
    }

    #[test]
    fn test_first_complex_grammar() {
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
            initial_symbol: 'S',
        };

        grammar.add_production(Production::new('E', "TZ".to_string()).unwrap());
        grammar.add_production(Production::new('Z', "+TZ | £".to_string()).unwrap());
        grammar.add_production(Production::new('T', "FY".to_string()).unwrap());
        grammar.add_production(Production::new('Y', "*FY | £".to_string()).unwrap());
        grammar.add_production(Production::new('F', "(E) | id".to_string()).unwrap());

        grammar.compute_firsts();

        let e = grammar.get_production_by_var('E').unwrap();
        let z = grammar.get_production_by_var('Z').unwrap();
        let t = grammar.get_production_by_var('T').unwrap();
        let y = grammar.get_production_by_var('Y').unwrap();
        let f = grammar.get_production_by_var('F').unwrap();

        let set_f = hash_from_vec(vec!["(", "id"]);
        let set_z = hash_from_vec(vec!["+", EPSILON]);
        let set_y = hash_from_vec(vec!["*", EPSILON]);

        assert_eq!(e.firsts, set_f, "Testing variable E");
        assert_eq!(z.firsts, set_z, "Testing variable Z");
        assert_eq!(t.firsts, set_f, "Testing variable T");
        assert_eq!(y.firsts, set_y, "Testing variable Y");
        assert_eq!(f.firsts, set_f, "Testing variable F");
    }
}
