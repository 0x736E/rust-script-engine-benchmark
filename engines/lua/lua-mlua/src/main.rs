use mlua::{Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();

    let sample  = include_str!("../../../../samples/sample.lua");

    let output: String = lua.load(sample).eval()?;
    println!("The answer is: {}", output);

    Ok(())
}