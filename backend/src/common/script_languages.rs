use rhai::{Scope, Engine};
use rlua::Lua;

const INPUT: &str = "input";

pub fn match_with_rhai(input: &str, script: &str) -> bool {
    let mut scope = Scope::new();

    scope.push(INPUT, input.to_owned());

    let engine = Engine::new();
    engine.eval_with_scope::<bool>(&mut scope, script).unwrap()
}

pub fn match_with_lua(input: &str, script: &str) -> bool {
    let lua = Lua::new();
    let mut result = false;

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals        
            .set(INPUT, wrap_in_brackets(input))
            .expect("Could not set global value");

        result = lua_ctx.load(script).eval().unwrap();
    });

    result
}

fn wrap_in_brackets(input: &str) -> String {
    format!("[[{}]]", input)
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


    #[test]
    fn test_with_rhai() {
        let input = "dies ist ein test";
        let script = "input.contains(\"test\")";

        assert_eq!(true, match_with_rhai(input, script));
    }
}