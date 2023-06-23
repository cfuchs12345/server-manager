use std::process::{Command, Output};

use serde::{Deserialize, Serialize};

use crate::{common, errors::Error};

#[derive(Debug, Serialize, Deserialize)]
struct CommandOutput {
    success: bool,
    stdout: String,
    stderr: String,
}

impl From<Output> for CommandOutput {
    fn from(output: Output) -> Self {
        CommandOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }
}

pub fn execute_command(command: &str) -> Result<Option<String>, Error> {
    let output = if *common::IS_WINDOWS {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .map_err(Error::from)
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(Error::from)
    }?;
    let json = serde_json::to_string(&CommandOutput::from(output))?;

    Ok(Some(json))
}
