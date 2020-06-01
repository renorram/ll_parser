use crate::grammar::Grammar;
use crate::production::Production;
use crate::token::Token;
use std::collections::HashSet;

impl Production {
    fn fetch_follows(production: &Production, grammar: &Grammar) -> HashSet<Token> {
        let mut follows: HashSet<Token> = HashSet::new();

        if grammar.production_is_initial(production) {
            follows.insert(Token::DollarSign);
        }

        follows
    }
}

impl Grammar {
    pub fn compute_follows(&mut self) {
        self.compute_firsts();

        let immut_self = self.clone();

        for p in self.productions_iter_mut() {
            p.set_follows(Production::fetch_follows(p, &immut_self))
        }
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
}
