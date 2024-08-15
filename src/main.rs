mod parser;

use parser::{Parser, Rule};
use pest::{error::Error, Parser as _};

fn main() -> Result<(), Error<parser::Rule>> {
    let tt = Parser::parse(Rule::src, include_str!("../tour/01-variables.idk"))?;
    let tlist = tt.flatten().tokens().collect::<Vec<_>>();
    println!("{tlist:?}");

    Ok(())
}
