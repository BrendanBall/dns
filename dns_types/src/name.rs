use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Name(pub Vec<String>);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for part in self.0.as_slice() {
            write!(f, "{}.", part)?;
        }
        Ok(())
    }
}
