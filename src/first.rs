use super::grammar::Grammar;
use super::production::{Production, EPSILON};

impl Production {
    fn fetch_firsts(production: &Production, grammar: &Grammar) -> Vec<String> {
        let mut firsts: Vec<String> = vec![];
        let mut buffer = String::new();

        for derivation_slice in production.get_derivation_slices() {
            // clean buffer before process a slice
            buffer.clear();
            for ch in derivation_slice.chars() {
                if grammar.variables.contains(&ch) {
                    if let Some(p) = grammar.get_production_by_var(ch) {
                        let variable_firsts = Self::fetch_firsts(p, grammar);
                        let should_continue = variable_firsts.contains(&EPSILON.to_string());

                        for v in variable_firsts {
                            if (v != EPSILON.to_string() && !firsts.contains(&v))
                                || (v == EPSILON.to_string() && derivation_slice.ends_with(ch))
                            {
                                firsts.push(v);
                            }
                        }

                        if !should_continue {
                            break;
                        }
                    }
                } else {
                    buffer.push(ch);

                    if (grammar.terminals.contains(&buffer) || buffer.eq(&EPSILON.to_string())) && !firsts.contains(&buffer) {
                        firsts.push(buffer.clone());
                        buffer.clear();
                        break;
                    }
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
    use crate::production::{Production, EPSILON};

    #[test]
    fn test_firsts_simple() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
        };
        
        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c".to_string()).unwrap());
        
        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        assert_eq!(s.firsts, vec!["a".to_string()], "Testing variable S");
        assert_eq!(a.firsts, vec!["a".to_string()], "Testing variable A");
        assert_eq!(b.firsts, vec!["b".to_string(), "c".to_string()], "Testing variable B");
    }

    #[test]
    fn test_firsts_epsilon() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
        };

        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a | £".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c".to_string()).unwrap());

        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        assert_eq!(s.firsts, vec!["a".to_string(), "b".to_string(), "c".to_string()], "Testing variable S");
        assert_eq!(a.firsts, vec!["a".to_string(), EPSILON.to_string()], "Testing variable A");
        assert_eq!(b.firsts, vec!["b".to_string(), "c".to_string()], "Testing variable B");
    }

    #[test]
    fn test_firsts_epsilon_2() {
        let mut grammar = Grammar {
            variables: vec!['S', 'A', 'B'],
            terminals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            productions: vec![],
        };

        grammar.add_production(Production::new('S', "AB".to_string()).unwrap());
        grammar.add_production(Production::new('A', "aA | a | £".to_string()).unwrap());
        grammar.add_production(Production::new('B', "bB | c | £".to_string()).unwrap());

        grammar.compute_firsts();

        let s = grammar.get_production_by_var('S').unwrap();
        let a = grammar.get_production_by_var('A').unwrap();
        let b = grammar.get_production_by_var('B').unwrap();
        assert_eq!(s.firsts, vec!["a".to_string(), "b".to_string(), "c".to_string(), EPSILON.to_string()], "Testing variable S");
        assert_eq!(a.firsts, vec!["a".to_string(), EPSILON.to_string()], "Testing variable A");
        assert_eq!(b.firsts, vec!["b".to_string(), "c".to_string(), EPSILON.to_string()], "Testing variable B");
    }

    #[test]
    fn test_first_complex_grammar() {
        let mut grammar = Grammar {
            variables: vec!['E', 'Z', 'T', 'Y', 'X'],
            terminals: vec!["+".to_string(), "*".to_string(), "(".to_string(), "id".to_string(), ")".to_string()],
            productions: vec![],
        };

        grammar.add_production(Production::new('E', "TZ".to_string()).unwrap());
        grammar.add_production(Production::new('Z', "+TZ | £".to_string()).unwrap());
        grammar.add_production(Production::new('T', "XY".to_string()).unwrap());
        grammar.add_production(Production::new('Y', "*XY | £".to_string()).unwrap());
        grammar.add_production(Production::new('X', "(E) | id".to_string()).unwrap());

        grammar.compute_firsts();

        let e = grammar.get_production_by_var('E').unwrap();
        let z = grammar.get_production_by_var('Z').unwrap();
        let t = grammar.get_production_by_var('T').unwrap();
        let y = grammar.get_production_by_var('Y').unwrap();
        let x = grammar.get_production_by_var('X').unwrap();
        assert_eq!(e.firsts, vec!["(".to_string(), "id".to_string()], "Testing variable E");
        assert_eq!(z.firsts, vec!["+".to_string(), EPSILON.to_string()], "Testing variable Z");
        assert_eq!(t.firsts, vec!["(".to_string(), "id".to_string()], "Testing variable T");
        assert_eq!(y.firsts, vec!["*".to_string(), EPSILON.to_string()], "Testing variable Y");
        assert_eq!(x.firsts, vec!["(".to_string(), "id".to_string()], "Testing variable X");
    }
}
