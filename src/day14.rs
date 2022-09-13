use crate::util::Result;

use std::collections::HashMap;

type Rules = HashMap<String, char>;

pub fn day_14() -> Result<()> {
    let contents = include_str!("../resources/day14_small.txt");
    println!("{}", contents);
    Ok(())
}

fn parse(content: &str) -> Result<(String, Rules)> {
    todo!("implement")
}
