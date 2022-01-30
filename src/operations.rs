use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub(crate) enum Operation {
    ADD,
    GET,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        match input.to_uppercase().as_str() {
            "ADD" => Ok(Operation::ADD),
            "GET" => Ok(Operation::GET),
            _ => Err(())
        }
    }
}