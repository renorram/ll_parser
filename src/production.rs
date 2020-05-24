use std::fmt;

pub const EPSILON: &str = "£";

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Production {
    pub variable: char,
    derivation: String,
    pub firsts: Vec<String>,
    follows: Vec<String>,
}

impl Production {
    pub fn new(variable: char, derivation: String) -> Result<Production, String> {
        if variable.is_uppercase() {
            return Ok(Production {
                variable,
                derivation: derivation.replace(' ', ""),
                firsts: vec![],
                follows: vec![],
            });
        }

        Err(format!(
            "the variable '{}' must be a uppercase character.",
            variable
        ))
    }

    pub fn set_firsts(&mut self, firsts: Vec<String>) {
        self.firsts = firsts;
    }

    pub fn get_derivation_slices(&self) -> std::str::Split<'_, &str> {
        self.derivation.split("|")
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{:^15}|{:^15}|{:^15}|{:^15}|",
            self.variable,
            self.derivation,
            self.firsts.join(","),
            self.follows.join(",")
        )
    }
}
