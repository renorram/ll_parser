use std::fmt;
use super::production::Production;

pub enum GrammarError {
    InvalidVariable
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grammar {
    pub variables: Vec<char>,
    pub terminals: Vec<String>,
    pub productions: Vec<Production>
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            variables: vec![],
            terminals: vec![],
            productions: vec![]
        }
    }

    pub fn add_variable(&mut self, variable: char) -> Result<(), GrammarError>{
        if !variable.is_uppercase() {
            return Err(GrammarError::InvalidVariable);
        }

        Ok(self.variables.push(variable))
    }

    pub fn add_terminal(&mut self, terminal: String) {
        self.terminals.push(terminal);
    }

    pub fn add_production(&mut self, production: Production) {
        if !self.productions.contains(&production) {
            self.productions.push(production);
        }
    }

    pub fn get_production_by_var(&self, variable: char) -> Option<&Production> {
        self.productions.iter().find(|p| p.variable == variable)
    }

    pub fn productions_iter_mut(&mut self) -> std::slice::IterMut<'_, Production> {
        self.productions.iter_mut()
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let divider: &str = &format!("*{:-<15}*{:-<15}*{:-<15}*{:-<15}*\n", "", "", "", "");

        f.write_str(divider)?;
        writeln!(
            f,
            "|{:^15}|{:^15}|{:^15}|{:^15}|",
            "Variable", "Derivation", "Firsts", "Follows"
        )?;
        f.write_str(divider)?;

        for production in self.productions.iter() {
            writeln!(f, "{}", production)?;
        }

        f.write_str(divider)
    }
}

#[cfg(test)]
mod test {
    use super::Grammar;

    #[test]
    fn test_grammar() {
        let grammar = Grammar::new();

        assert_eq!(grammar, Grammar {
            variables: vec![],
            terminals: vec![],
            productions: vec![]
        });
    }
}
