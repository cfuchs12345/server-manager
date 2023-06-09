use rhai::{Engine, Scope};
use rlua::Lua;

use crate::models::error::AppError;

const INPUT: &str = "input";

pub fn match_with_rhai(input: &str, script: &str) -> Result<bool, AppError> {
    let mut scope = Scope::new();

    scope.push(INPUT, input.to_owned());

    let engine = Engine::new();
    engine
        .eval_with_scope::<bool>(&mut scope, script)
        .map_err(|err| {
            AppError::ScriptError(format!(
                "Could not successfully execute rhai script {}. Error was {}",
                script, err
            ))
        })
}

pub fn match_with_lua(input: &str, script: &str) -> Result<bool, AppError> {
    let lua = Lua::new();
    let mut result = Ok(false);

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals
            .set(INPUT, wrap_in_brackets(input))
            .expect("Could not set global value");

        result = lua_ctx.load(script).eval().map_err(|err| {
            AppError::ScriptError(format!(
                "Could not successfuly execute lua script {}. Error was {}",
                script, err
            ))
        });
    });

    result
}

pub fn process_with_lua(input: &str, script: &str) -> Result<String, AppError> {
    let lua = Lua::new();
    let mut result: Result<String, AppError> = Ok("".to_string());

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals
            .set(INPUT, input)
            .expect("Could not set global value");

        result = lua_ctx
            .load(script)
            .eval()
            .map_err(|e| AppError::ScriptError(format!("{}", e)));
    });

    result
}

pub fn process_with_rhai(input: &str, script: &str) -> Result<String, AppError> {
    let mut scope = Scope::new();

    scope.push(INPUT, input.to_owned());

    let engine = Engine::new();
    engine
        .eval_with_scope::<String>(&mut scope, script)
        .map_err(|e| AppError::ScriptError(format!("{}", e)))
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

        assert!(match_with_lua(input, script).expect("should not happen"));
    }

    #[test]
    fn test_with_rhai() {
        let input = "dies ist ein test";
        let script = "input.contains(\"test\")";

        assert!(match_with_rhai(input, script).expect("should not happen"));
    }
}
