use std::collections::HashMap;
use std::process::{Command, ExitStatus};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct CommandStruct {
    pub command: String,
    pub args: Vec<String>,
    pub is_ok: bool,
    pub exit_code: i32,
    pub output: Vec<HashMap<String, Value>>
}

impl CommandStruct {
    pub fn new(command: String, args: Vec<String>) -> CommandStruct {
        CommandStruct {
            command,
            args,
            is_ok: false,
            exit_code: 0,
            output: vec![]
        }
    }
}

pub trait ContainerCommand {

    fn execute() -> Result<CommandStruct, String>;

}

pub struct DockerListRunningContainers;

impl ContainerCommand for DockerListRunningContainers {
    // const COMMAND_NAME: String = String::from("");
    // const COMMAND_ARGS: Vec<String> = vec![String::from("ps"), String::from("--format"), String::from("json")];

    fn execute() -> Result<CommandStruct, String> {
        let command_name = String::from("docker");
        let command_args = vec![String::from("ps"), String::from("--all"), String::from("--format"), String::from("json")];
        let mut command_struct = CommandStruct::new(command_name, command_args);
        let command_result = Command::new(command_struct.command.clone()).args(command_struct.args.clone()).output().map_err(|err| format!("Unexpected error: {}", err))?;

        if !command_result.status.success() {
            return Err(String::from_utf8(command_result.stderr).unwrap());
        }

        let stdout_string = String::from_utf8(command_result.stdout).unwrap();
        let stdout_lines = stdout_string.lines();
        command_struct.is_ok = command_result.status.success();
        command_struct.exit_code = command_result.status.code().unwrap();
        command_struct.output.clear();
        for line in stdout_lines {
            let x = serde_json::from_str(line);
            match x {
                Ok(serde_map) => command_struct.output.push(serde_map),
                Err(err) => {
                    println!("Erro: {}", err);
                    command_struct.output = vec![HashMap::from([(String::from("ran"), Value::Bool(true))])];
                }
            }
        }

        Ok(command_struct)
    }
}

struct DockerListAllContainers;

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn test_first() {
        let _t = DockerListRunningContainers::execute().unwrap();
        println!("Size is: {}", _t.output.len());
        assert!(true);
    }
}