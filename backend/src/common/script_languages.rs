use rhai::{Scope, Engine};
use rlua::Lua;


pub fn match_with_rhai(input: &str, script: &str) -> bool {
    let mut scope = Scope::new();

    scope.push("input", input.to_owned());

    let engine = Engine::new();
    engine.eval_with_scope::<bool>(&mut scope, script).unwrap()
}

pub fn match_with_lua(input: &str, script: &str) -> bool {
    let lua = Lua::new();
    let mut result = false;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals        
            .set("input", format!("[[{}]]", input))
            .expect("Could not set global value");

        result = lua_ctx.load(script).eval().unwrap();
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_lua() {
        let input = "dies ist ein test";
        let script = "string.find(input, 'test', 1, true)";

        assert_eq!(true, match_with_lua(input, script));
    }
}